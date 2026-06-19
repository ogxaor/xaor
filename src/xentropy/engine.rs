use rand::rngs::OsRng;
use rand::RngCore;

use crate::error::XaorError;

#[derive(Debug, Clone)]
pub struct EntropyVector {
    pub bytes: Vec<u8>,
}

impl EntropyVector {
    pub fn len(&self) -> usize {
        self.bytes.len()
    }
}

pub struct EntropyEngine {
    output_size: usize,
}

impl EntropyEngine {
    pub fn new(output_size: usize) -> Self {
        Self { output_size }
    }

    pub fn generate(&self) -> Result<EntropyVector, XaorError> {
        let mut buffer = vec![0u8; self.output_size];

        OsRng
            .try_fill_bytes(&mut buffer)
            .map_err(|_| XaorError::EntropyError)?;

        Ok(EntropyVector { bytes: buffer })
    }
}
