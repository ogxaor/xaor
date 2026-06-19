use xcrypt::engine::xcrypt::XcryptEngine;
use xcrypt::{config::XcryptMode, XcryptConfig};

fn main() {
    let mut config = XcryptConfig::default();
    config.mode = XcryptMode::Encrypt;
    let engine = XcryptEngine::new(config).unwrap();

    let result = engine.process(b"hello-world", None).unwrap();

    println!("Output length: {}", result.len());
    println!("{:?}", result);
}
