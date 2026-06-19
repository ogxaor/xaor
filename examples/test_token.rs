use xaor::TokenEngine;

fn main() {
    let engine = TokenEngine::default_session();

    let token = engine.generate().unwrap();
    let raw = engine.generate_bytes().unwrap();

    println!("Token: {}", token);
    println!("Raw length: {}", raw.len());
    println!("Hex: {}", TokenEngine::encode_hex(&raw));
}
