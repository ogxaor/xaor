use wasm_bindgen::prelude::*;
use crate::{
    Xaor, NonceEngine, TokenEngine, XidConfig, XidEngine, CipherEngine,
    VaultEngine, ProofConfig, ProofEngine, xproof::{ProofChallenge, ProofSolution}
};

#[wasm_bindgen]
pub fn hash_password(password: &str) -> Result<String, String> {
    let engine = Xaor::from_env().map_err(|e| format!("Config error: {}", e))?;
    engine.hash_password(password).map_err(|e| format!("Hashing failed: {}", e))
}

#[wasm_bindgen]
pub fn verify_password(password: &str, stored: &str) -> Result<bool, String> {
    let engine = Xaor::from_env().map_err(|e| format!("Config error: {}", e))?;
    engine.verify_password(password, stored).map_err(|e| format!("Verification failed: {}", e))
}

#[wasm_bindgen]
pub fn xnonce_generate(length: usize) -> Result<String, String> {
    let engine = NonceEngine::new(length).map_err(|e| format!("Nonce initialization error: {}", e))?;
    engine.generate_hex().map_err(|e| format!("Nonce generation error: {}", e))
}

#[wasm_bindgen]
pub fn xtoken_generate(length: usize, prefix: Option<String>) -> Result<String, String> {
    let engine = TokenEngine::with_prefix(length, prefix.as_deref())
        .map_err(|e| format!("Token initialization error: {}", e))?;
    engine.generate().map_err(|e| format!("Token generation error: {}", e))
}

#[wasm_bindgen]
pub fn xid_generate(length: usize, prefix: Option<String>, checksum: bool) -> Result<String, String> {
    let alphabet = crate::xid::config::default_alphabet().to_string();
    let config = XidConfig::with_options(length, alphabet, prefix, checksum)
        .map_err(|e| format!("Xid config error: {}", e))?;
    let engine = XidEngine::new(config).map_err(|e| format!("Xid engine error: {}", e))?;
    engine.generate().map_err(|e| format!("Xid generation error: {}", e))
}

#[wasm_bindgen]
pub fn xid_validate(id: &str, prefix: Option<String>, checksum: bool) -> bool {
    let alphabet = crate::xid::config::default_alphabet().to_string();
    let config = match XidConfig::with_options(21, alphabet, prefix, checksum) {
        Ok(c) => c,
        Err(_) => return false,
    };
    let engine = match XidEngine::new(config) {
        Ok(e) => e,
        Err(_) => return false,
    };
    matches!(engine.validate(id), Ok(true))
}

#[wasm_bindgen]
pub fn xcipher_encrypt(
    plaintext: &[u8],
    key: &[u8],
    nonce: &[u8],
    ad: Option<Vec<u8>>,
) -> Result<Vec<u8>, String> {
    let engine = CipherEngine::new();
    let ad_ref = ad.as_deref().unwrap_or(&[]);
    engine.encrypt(plaintext, key, nonce, ad_ref).map_err(|e| format!("Encryption failed: {}", e))
}

#[wasm_bindgen]
pub fn xcipher_decrypt(
    ciphertext: &[u8],
    key: &[u8],
    nonce: &[u8],
    ad: Option<Vec<u8>>,
) -> Result<Vec<u8>, String> {
    let engine = CipherEngine::new();
    let ad_ref = ad.as_deref().unwrap_or(&[]);
    engine.decrypt(ciphertext, key, nonce, ad_ref).map_err(|e| format!("Decryption failed: {}", e))
}

#[wasm_bindgen]
pub fn xvault_store(
    path: &str,
    name: &str,
    secret: &[u8],
    master_key: &[u8],
) -> Result<(), String> {
    let engine = VaultEngine::new(path);
    engine.store(name, secret, master_key).map_err(|e| format!("Store failed: {}", e))
}

#[wasm_bindgen]
pub fn xvault_retrieve(
    path: &str,
    name: &str,
    master_key: &[u8],
) -> Result<Option<Vec<u8>>, String> {
    let engine = VaultEngine::new(path);
    engine.retrieve(name, master_key).map_err(|e| format!("Retrieve failed: {}", e))
}

#[wasm_bindgen]
pub fn xproof_challenge(subject: &str, difficulty: u32) -> Result<String, String> {
    let config = ProofConfig {
        difficulty_bits: difficulty,
        ttl_secs: 300,
    };
    let engine = ProofEngine::new(config).map_err(|e| format!("Proof Engine config error: {}", e))?;
    let challenge = engine.challenge(subject).map_err(|e| format!("Proof challenge creation failed: {}", e))?;
    Ok(challenge.serialize())
}

#[wasm_bindgen]
pub fn xproof_solve(challenge_serialized: &str) -> Result<String, String> {
    let challenge = ProofChallenge::deserialize(challenge_serialized)
        .ok_or_else(|| "Failed to deserialize challenge".to_string())?;
    let engine = ProofEngine::default_low();
    let solution = engine.solve(&challenge).map_err(|e| format!("Proof solving failed: {}", e))?;
    Ok(solution.serialize())
}

#[wasm_bindgen]
pub fn xproof_verify(challenge_serialized: &str, solution_serialized: &str) -> Result<bool, String> {
    let challenge = ProofChallenge::deserialize(challenge_serialized)
        .ok_or_else(|| "Failed to deserialize challenge".to_string())?;
    let solution = ProofSolution::deserialize(solution_serialized)
        .ok_or_else(|| "Failed to deserialize solution".to_string())?;
    let engine = ProofEngine::default_low();
    engine.verify(&challenge, &solution).map_err(|e| format!("Proof verification failed: {}", e))
}
