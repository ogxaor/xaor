use xaor::engine::xaor::XaorEngine;
use xaor::{config::XaorMode, XaorConfig};

fn main() {
    let mut config = XaorConfig::default();
    config.mode = XaorMode::Encrypt;

    let engine = XaorEngine::new(config).unwrap();

    let seed = engine.generate_seed(b"password123", None).unwrap();

    println!("Seed length: {}", seed.len());
    println!("{:?}", seed);
}
