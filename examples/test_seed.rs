use xcrypt::engine::xcrypt::XcryptEngine;
use xcrypt::XcryptConfig;

fn main() {
    let config = XcryptConfig::default();

    let engine = XcryptEngine::new(config).unwrap();

    let seed = engine.generate_seed(b"password123").unwrap();

    println!("Seed length: {}", seed.len());
    println!("{:?}", seed);
}