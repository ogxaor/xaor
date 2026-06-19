use xcrypt::engine::xcrypt::XcryptEngine;
use xcrypt::{config::XcryptMode, XcryptConfig};

fn main() {
    let mut config = XcryptConfig::default();
    config.mode = XcryptMode::Encrypt;

    let engine = XcryptEngine::new(config).unwrap();

    let seed = engine.generate_seed(b"password123", None).unwrap();

    println!("Seed length: {}", seed.len());
    println!("{:?}", seed);
}
