/// Configuration for the ChaosEngine.
///
/// The integer chaos map requires no tunable parameters — it is fully
/// determined by the input seed. This struct is kept for API compatibility
/// and future extensibility (e.g., chaos rounds).
#[derive(Debug, Clone, Copy)]
pub struct ChaosConfig {
    /// Number of additional warm-up iterations before streaming.
    /// More warm-up = harder to predict initial state. Default: 8.
    pub warmup_rounds: u32,
}

impl Default for ChaosConfig {
    fn default() -> Self {
        Self { warmup_rounds: 8 }
    }
}
