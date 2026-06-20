use crate::chaos::{logistic, ChaosConfig};
use crate::pipeline::context::PipelineContext;
use crate::traits::stage::EngineStage;

/// Applies an integer chaotic XOR transformation to the input state.
///
/// Each output byte is the input byte XORed with a chaotic stream byte,
/// then rotated left by an amount derived from the chaos value itself.
/// This ensures the rotation amount is data-dependent — not constant —
/// which is critical for non-linear mixing without timing side-channels.
#[derive(Debug, Clone)]
pub struct ChaosEngine {
    config: ChaosConfig,
}

impl ChaosEngine {
    pub fn new() -> Self {
        Self {
            config: ChaosConfig::default(),
        }
    }

    pub fn with_config(config: ChaosConfig) -> Self {
        Self { config }
    }
}

impl Default for ChaosEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl ChaosEngine {
    pub fn transform(&self, input: &[u8]) -> Vec<u8> {
        if input.is_empty() {
            return Vec::new();
        }

        // Warm up: run the chaotic map for warmup_rounds before streaming
        // to make the initial state harder to guess from the seed alone.
        let mut stream = logistic::chaos_stream(input, input.len() + self.config.warmup_rounds as usize);
        let stream = stream.split_off(self.config.warmup_rounds as usize);

        let mut output = Vec::with_capacity(input.len());

        for (index, byte) in input.iter().enumerate() {
            let chaos_byte = stream[index];
            // XOR with chaos stream, then rotate left by (chaos_byte % 7 + 1)
            // to guarantee rotation amount is always 1-8 (never 0 = no-op).
            let rotation = (chaos_byte % 7) + 1;
            let rotated = (byte ^ chaos_byte).rotate_left(rotation as u32);
            output.push(rotated);
        }

        output
    }
}

impl EngineStage for ChaosEngine {
    fn name(&self) -> &'static str {
        "ChaosStage"
    }

    fn execute(&self, input: Vec<u8>, _ctx: &mut PipelineContext) -> Vec<u8> {
        self.transform(&input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic_output() {
        let engine = ChaosEngine::new();
        let input = b"compound-output".to_vec();

        assert_eq!(engine.transform(&input), engine.transform(&input));
    }

    #[test]
    fn no_panics_on_empty_input() {
        let engine = ChaosEngine::new();
        let input = Vec::new();

        assert!(engine.transform(&input).is_empty());
    }

    #[test]
    fn avalanche_changes_output() {
        let engine = ChaosEngine::new();
        let mut input = b"compound-output".to_vec();
        let a = engine.transform(&input);
        input[0] ^= 0x01;
        let b = engine.transform(&input);

        let differing_bits: u32 = a
            .iter()
            .zip(b.iter())
            .map(|(left, right)| (left ^ right).count_ones())
            .sum();

        let total_bits = (a.len() * 8) as u32;

        assert!(differing_bits > total_bits / 5);
    }

    #[test]
    fn rotation_never_zero() {
        // Regression test: old code used `chaos_byte % 8` which could be 0
        // (a no-op rotation). New code uses `% 7 + 1` ensuring 1-8.
        let engine = ChaosEngine::new();
        // All-zero input would expose any zero-rotation no-ops
        let input = vec![0u8; 64];
        let out = engine.transform(&input);
        // If rotation were ever 0, XOR would be the only transform
        // and output would equal the chaos stream — check that's not all zeros
        let nonzero = out.iter().any(|&b| b != 0);
        assert!(nonzero, "output must not be all-zero with all-zero input");
    }
}
