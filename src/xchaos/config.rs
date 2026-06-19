#[derive(Debug, Clone, Copy)]
pub struct ChaosConfig {
    pub scale: u64,
    pub r: u64,
}

impl Default for ChaosConfig {
    fn default() -> Self {
        Self {
            scale: 1_000_000,
            r: 3_990_000,
        }
    }
}
