use xaor::compound::CompoundEngine;
use xaor::pipeline::context::PipelineContext;
use xaor::topology::{Graph, Node, NodeType};
use xaor::traits::stage::EngineStage;

fn main() {
    // Construct a small manual graph for testing.
    // Node subkeys are derived from the password+salt in production —
    // here we use a fixed test subkey to exercise the CompoundEngine.
    let test_subkey_0: [u8; 32] = *blake3::hash(b"xaor.topology.v2.test.node.0").as_bytes();
    let test_subkey_1: [u8; 32] = *blake3::hash(b"xaor.topology.v2.test.node.1").as_bytes();

    let graph = Graph {
        nodes: vec![
            Node {
                id: 0,
                node_type: NodeType::Rotate,
                edges: vec![1],
                subkey: test_subkey_0,
            },
            Node {
                id: 1,
                node_type: NodeType::Xor,
                edges: vec![0],
                subkey: test_subkey_1,
            },
        ],
    };

    let engine = CompoundEngine::new(graph, 3);

    let input = vec![1u8; 64]; // 64-byte input (8 x 64-bit words)
    let mut ctx = PipelineContext::default();
    let output = engine.execute(input.clone(), &mut ctx);

    println!("Input:  {:?}", &input[..8]);
    println!("Output: {:?}", &output[..8]);
    println!("Outputs differ from input: {}", output != input);
}
