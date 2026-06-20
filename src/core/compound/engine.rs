use blake3::Hasher;

use crate::topology::{Graph, NodeType};
use crate::traits::stage::EngineStage;

/// BLAKE3-keyed compound transformation engine.
///
/// Each graph node carries a unique 32-byte subkey (derived from the
/// password+salt via BLAKE3 in TopologyEngine). CompoundEngine uses these
/// subkeys to perform genuinely SECRET-DEPENDENT operations per round.
///
/// Old behaviour: constant XOR, constant multiply, always rotate_left(1).
/// New behaviour: every 64-bit word transform is keyed by the node's subkey.
///
/// This means an attacker who knows the graph structure but not the password
/// cannot reproduce the compound transformation — it is no longer a public
/// permutation.
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
                // Derive per-node operation parameters from the 32-byte subkey.
                // These are secret-dependent because the subkey comes from
                // BLAKE3(domain || password_seed || node_index).
                let round_key =
                    u64::from_le_bytes(node.subkey[0..8].try_into().unwrap());
                // Rotation amount: derive from subkey, clamp to 1-63 (never 0 = no-op)
                let rotate_amt = (node.subkey[8] % 63) + 1;
                // Mix constant: must be odd for full-period multiplicative mixing
                let mix_const =
                    u64::from_le_bytes(node.subkey[9..17].try_into().unwrap()) | 1;

                match node.node_type {
                    NodeType::Rotate => {
                        // Rotate every 64-bit word by the node-derived amount.
                        // Old: always rotate_left(1) — trivially invertible.
                        // New: rotate_left(1-63), keyed per node per round.
                        for chunk in state.chunks_mut(8) {
                            if chunk.len() == 8 {
                                let mut w =
                                    u64::from_le_bytes(chunk.try_into().unwrap());
                                w = w.rotate_left(rotate_amt as u32);
                                chunk.copy_from_slice(&w.to_le_bytes());
                            }
                        }
                    }

                    NodeType::Xor => {
                        // XOR each 64-bit word with a node-derived keystream.
                        // The keystream value differs per word position (via
                        // `wrapping_add(i)`) so equal input words produce
                        // different outputs — breaks pattern repetition.
                        // Old: XOR all bytes with `(node.id as u8).wrapping_mul(31)`.
                        // New: each word gets a unique subkey-derived constant.
                        for (i, chunk) in state.chunks_mut(8).enumerate() {
                            if chunk.len() == 8 {
                                let mut w =
                                    u64::from_le_bytes(chunk.try_into().unwrap());
                                let ks = round_key
                                    .wrapping_add(i as u64)
                                    .rotate_left(((i * 3 + 1) % 64) as u32)
                                    .wrapping_mul(0x6C62272E07BB0142);
                                w ^= ks;
                                chunk.copy_from_slice(&w.to_le_bytes());
                            }
                        }
                    }

                    NodeType::Mix => {
                        // Multiply every 64-bit word by a node-derived ODD constant,
                        // then add the round key. Odd multiplier ensures the map is a
                        // bijection over Z/2^64, so every input maps to a unique output.
                        // Old: wrapping_add(i*17).rotate_left(3) — public, reversible.
                        // New: wrapping_mul(odd_subkey).wrapping_add(round_key) — keyed.
                        for chunk in state.chunks_mut(8) {
                            if chunk.len() == 8 {
                                let mut w =
                                    u64::from_le_bytes(chunk.try_into().unwrap());
                                w = w.wrapping_mul(mix_const).wrapping_add(round_key);
                                chunk.copy_from_slice(&w.to_le_bytes());
                            }
                        }
                    }

                    NodeType::Memory => {
                        // Full-state BLAKE3 diffusion keyed by this node's subkey.
                        // This is the most expensive node type — produces complete
                        // avalanche across all bytes of the state in one step.
                        // Old: target[idx] = target[idx].wrapping_mul(7).wrapping_add(13).
                        // New: XOR state with BLAKE3(subkey || state).
                        let mut hasher = Hasher::new();
                        hasher.update(b"xaor.compound.memory.v2");
                        hasher.update(&node.subkey);
                        hasher.update(&state);
                        let mut ks = vec![0u8; state.len()];
                        hasher.finalize_xof().fill(&mut ks);
                        for (s, k) in state.iter_mut().zip(ks.iter()) {
                            *s ^= k;
                        }
                    }
                }
            }
        }

        state
    }
}
