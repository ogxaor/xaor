use xcrypt::engine::xcrypt::XcryptEngine;
use xcrypt::{XcryptConfig, config::XcryptMode};

fn main() {
    let mut config = XcryptConfig::default();
    config.mode = XcryptMode::Hash;

    let engine = XcryptEngine::new(config).unwrap();

    let salt = b"fixed-salt-123";

    let a = engine.process(b"password123", Some(salt)).unwrap();
    let b = engine.process(b"password123", Some(salt)).unwrap();

    println!("{}", a == b);
}