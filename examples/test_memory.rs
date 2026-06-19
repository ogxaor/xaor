use xcrypt::engine::xcrypt::XcryptEngine;
use xcrypt::XcryptConfig;

fn main() {
    let config = XcryptConfig::default();
    let engine = XcryptEngine::new(config).unwrap();

    let result = engine.process(
        b"password123",
        Some(b"fixed-salt"),
    ).unwrap();

    println!("Length: {}", result.len());
    println!("{:?}", result);
}