#[derive(Debug, Clone)]
pub enum XcryptMode {
    Hash,
    Encrypt,
}

#[derive(Debug, Clone)]
pub struct XcryptConfig {
    pub rounds: usize,
    pub memory_size_mb: usize,
    pub node_count: usize,
    pub output_size: usize,
    pub mode: XcryptMode,
}

impl Default for XcryptConfig {
    fn default() -> Self {
        Self {
            rounds: 16,
            memory_size_mb: 256,
            node_count: 32,
            output_size: 64,
            mode: XcryptMode::Hash,
        }
    }
}

impl XcryptConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.rounds == 0 {
            return Err("rounds must be > 0".into());
        }

        if self.memory_size_mb < 8 {
            return Err("memory too small".into());
        }

        if self.node_count == 0 {
            return Err("node count must be > 0".into());
        }

        if self.output_size == 0 {
            return Err("output size must be > 0".into());
        }

        Ok(())
    }
}