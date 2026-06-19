use xcrypt::engine::xcrypt::XcryptEngine;
use xcrypt::{config::XcryptMode, XcryptConfig};

fn main() {
    let mut config = XcryptConfig::default();
    config.mode = XcryptMode::Encrypt;
    let engine = XcryptEngine::new(config).unwrap();

    let result = engine.process(b"xcrypt-test", None).unwrap();

    println!("Final length: {}", result.len());
    println!("{:?}", result);
}
