use blake3::Hasher;

use crate::topology::nodes::NodeType;

/// A node in Xaor's dynamic computation graph.
///
/// Each node carries a unique 32-byte subkey derived from the password+salt
/// via BLAKE3. The CompoundEngine uses this subkey to perform genuinely
/// secret-dependent operations — unlike the old design that used public
/// constant operations (XOR with a fixed constant, etc.).
#[derive(Debug, Clone)]
pub struct GraphNode {
    pub id: usize,
    pub node_type: NodeType,
    pub edges: Vec<usize>,
    /// 32-byte secret subkey derived from BLAKE3(domain || seed || node_index).
    /// Used by CompoundEngine for keyed 64-bit word transforms.
    pub subkey: [u8; 32],
}

#[derive(Debug, Clone)]
pub struct Graph {
    pub nodes: Vec<GraphNode>,
}

pub struct TopologyEngine;

impl TopologyEngine {
    pub fn new() -> Self {
        Self
    }

    /// Generate a data-dependent computation graph from `seed`.
    ///
    /// Every node's type, edges, and subkey are derived from a full BLAKE3
    /// digest of `(domain || seed || node_index)`, giving 2^256 possible
    /// topologies per node — not the 256 that the old single-byte derivation
    /// allowed.
    pub fn generate(seed: &[u8], node_count: usize) -> Graph {
        if node_count == 0 {
            return Graph { nodes: Vec::new() };
        }

        let mut nodes = Vec::with_capacity(node_count);

        for i in 0..node_count {
            // Derive a unique 32-byte digest for each node.
            let mut hasher = Hasher::new();
            hasher.update(b"xaor.topology.v2");
            hasher.update(seed);
            hasher.update(&(i as u64).to_le_bytes());
            let node_hash = hasher.finalize();
            let h = node_hash.as_bytes();

            // Node type from first byte
            let node_type = match h[0] % 4 {
                0 => NodeType::Rotate,
                1 => NodeType::Xor,
                2 => NodeType::Mix,
                _ => NodeType::Memory,
            };

            // Edge targets from independent 32-bit regions of the hash
            let edge_a = u32::from_le_bytes([h[1], h[2], h[3], h[4]]) as usize % node_count;
            let edge_b = u32::from_le_bytes([h[5], h[6], h[7], h[8]]) as usize % node_count;

            // Full 32-byte subkey from the hash — used by CompoundEngine for keyed ops
            let mut subkey = [0u8; 32];
            subkey.copy_from_slice(h);

            nodes.push(GraphNode {
                id: i,
                node_type,
                edges: vec![edge_a, edge_b],
                subkey,
            });
        }

        Graph { nodes }
    }
}

impl Default for TopologyEngine {
    fn default() -> Self {
        Self::new()
    }
}
