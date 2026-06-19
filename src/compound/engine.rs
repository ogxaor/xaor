use crate::topology::{Graph, NodeType};

pub struct CompoundEngine;

impl CompoundEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn process(seed: &[u8], graph: &Graph, rounds: usize) -> Vec<u8> {
        let mut state = seed.to_vec();

        for _ in 0..rounds {
            for node in &graph.nodes {
                match node.node_type {
                    NodeType::Rotate => {
                        state.rotate_left(1);
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
                        let idx = node.id % state.len();
                        state[idx] = state[idx].wrapping_mul(7).wrapping_add(13);
                    }
                }
            }
        }

        state
    }
}