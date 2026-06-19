use xcrypt::NonceEngine;

fn main() {
    let engine = NonceEngine::default_128();

    let nonce_bytes = engine.generate().unwrap();
    let nonce_hex = NonceEngine::encode_hex(&nonce_bytes);
    let nonce_b64 = NonceEngine::encode_base64(&nonce_bytes);

    println!("Nonce bytes: {}", nonce_bytes.len());
    println!("Nonce hex: {}", nonce_hex);
    println!("Nonce base64: {}", nonce_b64);
}
