use crate::chaos::{logistic, ChaosConfig};
use crate::pipeline::context::PipelineContext;
use crate::traits::stage::EngineStage;

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

    pub fn transform(&self, input: &[u8]) -> Vec<u8> {
        if input.is_empty() {
            return Vec::new();
        }

        let chaos = logistic::chaos_stream(input, input.len(), self.config);
        let mut output = Vec::with_capacity(input.len());

        for (index, byte) in input.iter().enumerate() {
            let chaos_byte = chaos[index];
            let rotated = (byte ^ chaos_byte).rotate_left((chaos_byte % 8) as u32);
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
}
