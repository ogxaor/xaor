use crate::compound::CompoundEngine;
use crate::constant_time_eq;
use crate::finalizer::FinalizerEngine;
use crate::memory::MemoryArena;
use crate::recycler::RecyclerEngine;
use crate::seed::SeedEngine;
use crate::topology::TopologyEngine;
use crate::{XcryptConfig, XcryptError};
use base64::{engine::general_purpose, Engine as _};

pub struct XcryptEngine {
    config: XcryptConfig,
}

impl XcryptEngine {
    pub fn new(config: XcryptConfig) -> Result<Self, XcryptError> {
        config
            .validate()
            .map_err(crate::error::XcryptError::InvalidConfig)?;

        Ok(Self { config })
    }

    pub fn generate_seed(&self, input: &[u8], salt: Option<&[u8]>) -> Result<Vec<u8>, XcryptError> {
        let entropy = match self.config.mode {
            crate::config::XcryptMode::Hash => {
                let salt = salt.ok_or(XcryptError::SeedError)?;

                crate::entropy::EntropyVector {
                    bytes: salt.to_vec(),
                }
            }

            crate::config::XcryptMode::Encrypt => {
                let entropy_engine = crate::entropy::EntropyEngine::new(self.config.output_size);

                entropy_engine.generate()?
            }
        };

        let seed_engine = SeedEngine::new();
        let seed = seed_engine.generate(input, &entropy)?;

        Ok(seed.root.to_vec())
    }

    pub fn process(&self, input: &[u8], salt: Option<&[u8]>) -> Result<Vec<u8>, XcryptError> {
        let seed = self.generate_seed(input, salt)?;
        let graph = TopologyEngine::generate(&seed, self.config.node_count);

        let compound = CompoundEngine::process(&seed, &graph, self.config.rounds);

        let recycled = RecyclerEngine::recycle(&compound, self.config.rounds);

        let memory_hardened = MemoryArena::process(&recycled, self.config.memory_size_mb);

        let final_output = FinalizerEngine::finalize(&memory_hardened);
        Ok(final_output)
    }

    pub fn hash_password(&self, password: &str) -> Result<String, XcryptError> {
        use rand::RngCore;

        let mut salt = [0u8; 16];
        rand::thread_rng().fill_bytes(&mut salt);

        let hash = self.process(password.as_bytes(), Some(&salt))?;

        let salt_b64 = general_purpose::STANDARD.encode(salt);
        let hash_b64 = general_purpose::STANDARD.encode(hash);

        Ok(format!(
            "$xcrypt$v=1$m={}$r={}$n={}${}${}",
            self.config.memory_size_mb,
            self.config.rounds,
            self.config.node_count,
            salt_b64,
            hash_b64
        ))
    }

    pub fn verify_password(&self, password: &str, stored: &str) -> Result<bool, XcryptError> {
        let parts: Vec<&str> = stored.split('$').collect();

        if parts.len() < 7 {
            return Err(XcryptError::SeedError);
        }

        let salt_b64 = parts[6];
        let hash_b64 = parts[7];

        let salt = general_purpose::STANDARD
            .decode(salt_b64)
            .map_err(|_| XcryptError::SeedError)?;

        let stored_hash = general_purpose::STANDARD
            .decode(hash_b64)
            .map_err(|_| XcryptError::SeedError)?;

        let computed = self.process(password.as_bytes(), Some(&salt))?;

        Ok(constant_time_eq(&computed, &stored_hash))
    }
}
