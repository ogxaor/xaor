use crate::topology::nodes::NodeType;

#[derive(Debug, Clone)]
pub struct GraphNode {
    pub id: usize,
    pub node_type: NodeType,
    pub edges: Vec<usize>,
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

    pub fn generate(seed: &[u8], node_count: usize) -> Graph {
        if node_count == 0 {
            return Graph { nodes: Vec::new() };
        }

        let mut nodes = Vec::with_capacity(node_count);

        for i in 0..node_count {
            let seed_byte = if seed.is_empty() {
                0
            } else {
                seed[i % seed.len()]
            };

            let node_type = match seed_byte % 4 {
                0 => NodeType::Rotate,
                1 => NodeType::Xor,
                2 => NodeType::Mix,
                _ => NodeType::Memory,
            };

            let next = (i + 1) % node_count;
            let extra = (seed_byte as usize) % node_count;

            nodes.push(GraphNode {
                id: i,
                node_type,
                edges: vec![next, extra],
            });
        }

        Graph { nodes }
    }
}
