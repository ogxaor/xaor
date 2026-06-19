use xaor::engine::xaor::XaorEngine;
use xaor::{XaorConfig, config::XaorMode};

fn main() {
    let mut config = XaorConfig::default();
    config.mode = XaorMode::Hash;

    let engine = XaorEngine::new(config).unwrap();

    let salt = b"fixed-salt-123";

    let a = engine.process(b"password123", Some(salt)).unwrap();
    let b = engine.process(b"password123", Some(salt)).unwrap();

    println!("{}", a == b);
}
