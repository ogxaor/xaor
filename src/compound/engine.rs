use crate::topology::{Graph, NodeType};
use crate::traits::stage::EngineStage;

pub struct CompoundEngine {
    graph: Graph,
    rounds: usize,
}

impl CompoundEngine {
    pub fn new(graph: Graph, rounds: usize) -> Self {
        Self { graph, rounds }
    }
}

impl EngineStage for CompoundEngine {
    fn name(&self) -> &'static str {
        "CompoundEngine"
    }

    fn execute(
        &self,
        input: Vec<u8>,
        _ctx: &mut crate::pipeline::context::PipelineContext,
    ) -> Vec<u8> {
        let mut state = input;

        for _ in 0..self.rounds {
            for node in &self.graph.nodes {
                match node.node_type {
                    NodeType::Rotate => {
                        if !state.is_empty() {
                            state.rotate_left(1);
                        }
                    }

                    NodeType::Xor => {
                        for i in 0..state.len() {
                            state[i] ^= (node.id as u8).wrapping_mul(31);
                        }
                    }

                    NodeType::Mix => {
                        for i in 0..state.len() {
                            state[i] = state[i]
                                .wrapping_add((i as u8).wrapping_mul(17))
                                .rotate_left(3);
                        }
                    }

                    NodeType::Memory => {
                        if !state.is_empty() {
                            let idx = node.id % state.len();
                            state[idx] = state[idx].wrapping_mul(7).wrapping_add(13);
                        }
                    }
                }
            }
        }

        state
    }
}
