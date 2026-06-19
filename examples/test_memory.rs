use xaor::engine::xaor::XaorEngine;
use xaor::XaorConfig;

fn main() {
    let config = XaorConfig::default();
    let engine = XaorEngine::new(config).unwrap();

    let result = engine.process(
        b"password123",
        Some(b"fixed-salt"),
    ).unwrap();

    println!("Length: {}", result.len());
    println!("{:?}", result);
}
