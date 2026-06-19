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
        let seed = seed_engine.generate(input, &entropy)?;

        Ok(seed.root.to_vec())
    }

    pub fn process(
        &self,
        input: &[u8],
        salt: Option<&[u8]>,
    ) -> Result<Vec<u8>, XaorError> {
        let entropy = self.build_entropy(salt)?;

        let mut pipeline = PipelineRunner::new();
        pipeline.add_stage(Box::new(SeedStage::new(entropy)));
        pipeline.add_stage(Box::new(TopologyStage::new(self.config.node_count)));
        pipeline.add_stage(Box::new(CompoundStage::new(self.config.rounds)));
        pipeline.add_stage(Box::new(ChaosStage::new()));
        pipeline.add_stage(Box::new(RecyclerStage::new(self.config.rounds)));
        pipeline.add_stage(Box::new(MemoryStage::new(self.config.memory_size_mb)));
        pipeline.add_stage(Box::new(FinalizerStage::new()));

        Ok(pipeline.run(input.to_vec()))
    }

    pub fn hash_password(&self, password: &str) -> Result<String, XaorError> {
        let mut salt = [0u8; 16];
        rand::thread_rng().fill_bytes(&mut salt);

        let hash = self.process(password.as_bytes(), Some(&salt))?;

        let salt_b64 = general_purpose::STANDARD.encode(salt);
        let hash_b64 = general_purpose::STANDARD.encode(hash);

        Ok(format!(
            "$xaor$v=1$m={}$r={}$n={}${}${}",
            self.config.memory_size_mb,
            self.config.rounds,
            self.config.node_count,
            salt_b64,
            hash_b64
        ))
    }

    pub fn verify_password(&self, password: &str, stored: &str) -> Result<bool, XaorError> {
        let stored = StoredHash::parse(stored)?;
        let computed = self.process(password.as_bytes(), Some(&stored.salt))?;

        Ok(constant_time_eq(&computed, &stored.hash))
    }
}
