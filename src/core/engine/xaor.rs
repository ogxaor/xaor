use base64::{engine::general_purpose, Engine as _};
use rand::RngCore;

use crate::entropy::{EntropyEngine, EntropyVector};
use crate::pipeline::{
    ChaosStage, CompoundStage, FinalizerStage, MemoryStage, PipelineRunner, RecyclerStage,
    SeedStage, TopologyStage,
};
use crate::serialization::StoredHash;
use crate::seed::SeedEngine;
use crate::{constant_time_eq, XaorConfig, XaorError};
use crate::config::XaorOutputMode;

#[derive(Debug, Clone)]
pub struct XaorEngine {
    config: XaorConfig,
}

impl XaorEngine {
    pub fn new(config: XaorConfig) -> Result<Self, XaorError> {
        config
            .validate()
            .map_err(crate::error::XaorError::InvalidConfig)?;

        Ok(Self { config })
    }

    fn build_entropy(&self, salt: Option<&[u8]>) -> Result<EntropyVector, XaorError> {
        match self.config.mode {
            crate::config::XaorMode::Hash => {
                let salt = salt.ok_or(XaorError::SeedError)?;
                Ok(EntropyVector {
                    bytes: salt.to_vec(),
                })
            }
            crate::config::XaorMode::Encrypt => {
                let entropy_engine = EntropyEngine::new(self.config.output_size);
                entropy_engine.generate()
            }
        }
    }

    pub fn generate_seed(
        &self,
        input: &[u8],
        salt: Option<&[u8]>,
    ) -> Result<Vec<u8>, XaorError> {
        let entropy = self.build_entropy(salt)?;
        let seed_engine = SeedEngine::new();
        let seed = seed_engine.generate(input, &entropy, self.config.pepper.as_deref())?;

        Ok(seed.root.to_vec())
    }

    pub fn process(
        &self,
        input: &[u8],
        salt: Option<&[u8]>,
    ) -> Result<Vec<u8>, XaorError> {
        let entropy = self.build_entropy(salt)?;
        let output_size = self.config.output_mode.byte_length();

        let mut pipeline = PipelineRunner::new();

        // SeedStage: derives 64-byte root from input+salt, with optional pepper
        let seed_stage = match &self.config.pepper {
            Some(p) => SeedStage::with_pepper(entropy, p.clone()),
            None    => SeedStage::new(entropy),
        };
        pipeline.add_stage(Box::new(seed_stage));

        // TopologyStage: generates BLAKE3-keyed computation graph
        pipeline.add_stage(Box::new(TopologyStage::new(self.config.node_count)));

        // CompoundStage: BLAKE3-keyed 64-bit word transforms per graph node
        pipeline.add_stage(Box::new(CompoundStage::new(self.config.rounds)));

        // ChaosStage: integer xorshift chaos mixing (no f64 timing side-channel)
        pipeline.add_stage(Box::new(ChaosStage::new()));

        // RecyclerStage: BLAKE3-XOF keyed recycling with 64-bit word diffusion
        pipeline.add_stage(Box::new(RecyclerStage::new(self.config.rounds)));

        // MemoryStage: 3-pass memory-hard arena (forward+backward, keyed, zeroized)
        // Divide memory equally among lanes
        let lane_memory_mb = (self.config.memory_size_mb / self.config.lanes).max(1);
        pipeline.add_stage(Box::new(MemoryStage::new(lane_memory_mb)));

        // FinalizerStage: 64-byte standard or 128-byte quantum dual-XOF output
        pipeline.add_stage(Box::new(FinalizerStage::new(output_size)));

        let raw_output = pipeline.run(input.to_vec(), self.config.lanes);

        // If we ran multiple lanes, the raw_output is concatenated `L * output_size` bytes.
        // We compress it down to the final `output_size` using a final BLAKE3-XOF.
        if self.config.lanes > 1 {
            let mut hasher = blake3::Hasher::new();
            hasher.update(b"xaor.lanes.merge.v3");
            hasher.update(&raw_output);
            let mut final_output = vec![0u8; output_size];
            hasher.finalize_xof().fill(&mut final_output);
            Ok(final_output)
        } else {
            Ok(raw_output)
        }
    }

    pub fn hash_password(&self, password: &str) -> Result<String, XaorError> {
        let mut salt = [0u8; 32]; // 256-bit salt — 2x the old 128-bit salt
        rand::thread_rng().fill_bytes(&mut salt);

        let hash = self.process(password.as_bytes(), Some(&salt))?;

        let salt_b64 = general_purpose::STANDARD.encode(salt);
        let hash_b64 = general_purpose::STANDARD.encode(&hash);

        // Hash string format: $xaor$v=3$m=<MB>$r=<rounds>$n=<nodes>$l=<lanes>$o=<output_mode>$<salt>$<hash>
        let output_mode_str = match self.config.output_mode {
            XaorOutputMode::Standard => "std",
            XaorOutputMode::Quantum  => "qnt",
        };

        Ok(format!(
            "$xaor$v=3$m={}$r={}$n={}$l={}$o={}${}${}",
            self.config.memory_size_mb,
            self.config.rounds,
            self.config.node_count,
            self.config.lanes,
            output_mode_str,
            salt_b64,
            hash_b64,
        ))
    }

    pub fn verify_password(&self, password: &str, stored: &str) -> Result<bool, XaorError> {
        let stored = StoredHash::parse(stored)?;
        let computed = self.process(password.as_bytes(), Some(&stored.salt))?;

        Ok(constant_time_eq(&computed, &stored.hash))
    }
}
