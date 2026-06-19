use xaor::CipherEngine;

fn main() {
    let engine = CipherEngine::new();
    let key = b"0123456789abcdef0123456789abcdef"; // 32 bytes
    let nonce = b"abcdefghijkl"; // 12 bytes
    let plaintext = b"This is a highly confidential message for testing the new experimental xcipher engine.";
    let associated_data = b"version=1,author=antigravity";

    println!("Plaintext: '{}'", String::from_utf8_lossy(plaintext));
    println!("Key: {}", String::from_utf8_lossy(key));
    println!("Nonce: {}", String::from_utf8_lossy(nonce));
    println!("AD: {}", String::from_utf8_lossy(associated_data));

    // Encrypt
    let ciphertext = engine
        .encrypt(plaintext, key, nonce, associated_data)
        .expect("Encryption failed");

    println!("Encrypted data length: {} bytes (includes 32-byte tag)", ciphertext.len());
    println!("Encrypted hex: {}", hex::encode(&ciphertext));

    // Decrypt
    let decrypted = engine
        .decrypt(&ciphertext, key, nonce, associated_data)
        .expect("Decryption failed");

    println!("Decrypted plaintext: '{}'", String::from_utf8_lossy(&decrypted));

    assert_eq!(plaintext.as_slice(), decrypted.as_slice());
    println!("Verification successful: decrypted plaintext matches original!");
}
