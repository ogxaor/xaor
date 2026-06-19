use xcrypt::compound::CompoundEngine;
use xcrypt::pipeline::context::PipelineContext;
use xcrypt::topology::{Graph, Node, NodeType};
use xcrypt::traits::stage::EngineStage;

fn main() {
    let graph = Graph {
        nodes: vec![
            Node {
                id: 0,
                node_type: NodeType::Rotate,
                edges: vec![1],
            },
            Node {
                id: 1,
                node_type: NodeType::Xor,
                edges: vec![0],
            },
        ],
    };

    let engine = CompoundEngine::new(graph, 3);

    let input = vec![1, 2, 3, 4];
    let mut ctx = PipelineContext::default();
    let output = engine.execute(input, &mut ctx);

    println!("{:?}", output);
}
