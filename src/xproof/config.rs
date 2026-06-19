#[derive(Debug, Clone, Copy)]
pub struct ProofConfig {
    pub difficulty_bits: u32,
    pub ttl_secs: u64,
}

impl Default for ProofConfig {
    fn default() -> Self {
        Self {
            difficulty_bits: 16,
            ttl_secs: 300,
        }
    }
}

impl ProofConfig {
    pub fn with_difficulty(difficulty_bits: u32) -> Result<Self, String> {
        Self::with_options(difficulty_bits, 300)
    }

    pub fn with_options(difficulty_bits: u32, ttl_secs: u64) -> Result<Self, String> {
        if difficulty_bits == 0 {
            return Err("difficulty_bits must be greater than zero".into());
        }

        if difficulty_bits > 256 {
            return Err("difficulty_bits must be 256 or less".into());
        }

        if ttl_secs == 0 {
            return Err("ttl_secs must be greater than zero".into());
        }

        Ok(Self {
            difficulty_bits,
            ttl_secs,
        })
    }

    pub fn low() -> Self {
        Self {
            difficulty_bits: 12,
            ttl_secs: 120,
        }
    }

    pub fn medium() -> Self {
        Self {
            difficulty_bits: 18,
            ttl_secs: 300,
        }
    }

    pub fn high() -> Self {
        Self {
            difficulty_bits: 24,
            ttl_secs: 600,
        }
    }
}
