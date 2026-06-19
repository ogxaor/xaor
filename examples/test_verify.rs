use xcrypt::engine::xcrypt::XcryptEngine;
use xcrypt::XcryptConfig;

fn main() {
    let config = XcryptConfig::default();
    let engine = XcryptEngine::new(config).unwrap();

    let stored = engine
        .hash_password("super-secret")
        .unwrap();

    println!("Stored:");
    println!("{}", stored);

    let ok = engine
        .verify_password("super-secret", &stored)
        .unwrap();

    println!("Verified: {}", ok);
}