use xaor::entropy::EntropyEngine;

fn main() {
    let engine = EntropyEngine::new(64);

    let entropy = engine.generate().unwrap();

    println!("Entropy size: {}", entropy.len());
    println!("{:?}", entropy.bytes);
}
