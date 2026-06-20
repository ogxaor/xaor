use rand::rngs::OsRng;
use rand::RngCore;
use zeroize::ZeroizeOnDrop;

use crate::error::XaorError;

/// A vector of cryptographic entropy bytes.
///
/// Automatically zeroed from memory when dropped — prevents recovery of the
/// salt or encryption entropy from freed heap allocations.
#[derive(Debug, Clone, ZeroizeOnDrop)]
pub struct EntropyVector {
    pub bytes: Vec<u8>,
}

impl EntropyVector {
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }
}

pub struct EntropyEngine {
    output_size: usize,
}

impl EntropyEngine {
    pub fn new(output_size: usize) -> Self {
        Self { output_size }
    }

    /// Generate `output_size` bytes of OS-provided cryptographic randomness.
    pub fn generate(&self) -> Result<EntropyVector, XaorError> {
        let mut buffer = vec![0u8; self.output_size];

        OsRng
            .try_fill_bytes(&mut buffer)
            .map_err(|_| XaorError::EntropyError)?;

        Ok(EntropyVector { bytes: buffer })
    }
}
