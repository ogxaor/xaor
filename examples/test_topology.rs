use xaor::topology::TopologyEngine;

fn main() {
    let seed = vec![10, 20, 30, 40, 50, 60];
    let graph = TopologyEngine::generate(&seed, 8);

    for node in graph.nodes {
        println!("{:?}", node);
    }
}
