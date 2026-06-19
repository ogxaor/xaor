use xcrypt::engine::xcrypt::XcryptEngine;
use xcrypt::XcryptConfig;

fn main() {
    let config = XcryptConfig::default();
    let engine = XcryptEngine::new(config).unwrap();

    let result = engine.process(b"hello-xcrypt").unwrap();

    println!("Output length: {}", result.len());
    println!("{:?}", result);
}