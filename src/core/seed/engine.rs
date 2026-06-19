use blake3::Hasher;

use crate::entropy::EntropyVector;
use crate::error::XaorError;

#[derive(Debug, Clone)]
pub struct Seed {
    pub root: [u8; 64],
}

pub struct SeedEngine;

impl SeedEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn generate(
        &self,
        input: &[u8],
        entropy: &EntropyVector,
    ) -> Result<Seed, XaorError> {
        let mut hasher = Hasher::new();

        hasher.update(input);
        hasher.update(&entropy.bytes);

        let hash = hasher.finalize();

        let mut root = [0u8; 64];

        root[..32].copy_from_slice(hash.as_bytes());
        root[32..].copy_from_slice(hash.as_bytes());

        Ok(Seed { root })
    }
}
