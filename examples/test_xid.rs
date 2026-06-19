use xaor::xid::config::{default_alphabet, XidConfig};
use xaor::XidEngine;

fn main() {
    let random_engine = XidEngine::default_uuidish();
    let random_id = random_engine.generate().unwrap();

    let config = XidConfig::with_options(20, default_alphabet(), Some("xid_"), true).unwrap();
    let deterministic_engine = XidEngine::new(config).unwrap();
    let deterministic_a = deterministic_engine.generate_deterministic(b"customer-42").unwrap();
    let deterministic_b = deterministic_engine.generate_deterministic(b"customer-42").unwrap();

    println!("Random ID: {}", random_id);
    println!("Deterministic A: {}", deterministic_a);
    println!("Deterministic B: {}", deterministic_b);
    println!("Same deterministic output: {}", deterministic_a == deterministic_b);
}
