use xcrypt::engine::xcrypt::XcryptEngine;
use xcrypt::XcryptConfig;

fn main() {
    let config = XcryptConfig::default();
    let engine = match XcryptEngine::new(config) {
        Ok(engine) => engine,
        Err(err) => {
            eprintln!("failed to create engine: {} ({})", err, err.code());
            return;
        }
    };

    let stored = match engine.hash_password("super-secret") {
        Ok(stored) => stored,
        Err(err) => {
            eprintln!("hashing failed: {} ({})", err, err.code());
            return;
        }
    };

    println!("Stored:");
    println!("{}", stored);

    match engine.verify_password("super-secret", &stored) {
        Ok(ok) => println!("Verified: {}", ok),
        Err(err) => eprintln!("verification failed: {} ({})", err, err.code()),
    }
}
