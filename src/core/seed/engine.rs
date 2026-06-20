use blake3::Hasher;
use zeroize::ZeroizeOnDrop;

use crate::entropy::EntropyVector;
use crate::error::XaorError;

/// A cryptographic seed derived from the password, salt, and optional pepper.
///
/// The 64-byte `root` is automatically zeroed from memory when this struct
/// is dropped, preventing recovery of the seed from freed heap allocations.
#[derive(Debug, Clone, ZeroizeOnDrop)]
pub struct Seed {
    pub root: [u8; 64],
}

pub struct SeedEngine;

impl SeedEngine {
    pub fn new() -> Self {
        Self
    }

    /// Derive a 64-byte seed from the input, entropy (salt), and optional pepper.
    ///
    /// Uses BLAKE3-XOF with domain separation to produce all 64 bytes as
    /// genuinely independent keystream material (fixes the old duplicate-digest bug).
    ///
    /// The optional `pepper` is a server-side secret (never stored in the DB).
    /// If present, it is mixed in after the salt with its own domain label, making
    /// offline cracking impossible even if the database (hash+salt) is leaked.
    pub fn generate(
        &self,
        input: &[u8],
        entropy: &EntropyVector,
        pepper: Option<&[u8]>,
    ) -> Result<Seed, XaorError> {
        let mut hasher = Hasher::new();
        hasher.update(b"xaor.seed.v2");
        hasher.update(input);
        hasher.update(&entropy.bytes);

        if let Some(p) = pepper {
            // Domain-separated pepper prevents length-extension leaks
            hasher.update(b"xaor.seed.pepper.v1");
            hasher.update(p);
        }

        let mut reader = hasher.finalize_xof();
        let mut root = [0u8; 64];
        reader.fill(&mut root);

        Ok(Seed { root })
    }
}

impl Default for SeedEngine {
    fn default() -> Self {
        Self::new()
    }
}

