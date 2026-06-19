//! Experimental symmetric cipher providing authenticated encryption (AEAD).

use crate::{constant_time_eq, XaorError};

#[derive(Debug, Clone, Default)]
pub struct CipherEngine;

impl CipherEngine {
    /// Create a new CipherEngine instance.
    pub fn new() -> Self {
        Self
    }

    /// Encrypt a plaintext payload with a key, a nonce, and optional associated data.
    ///
    /// The returned Vec contains the ciphertext followed by the 32-byte authentication tag.
    pub fn encrypt(
        &self,
        plaintext: &[u8],
        key: &[u8],
        nonce: &[u8],
        associated_data: &[u8],
    ) -> Result<Vec<u8>, XaorError> {
        validate_key_and_nonce(key, nonce)?;

        let ciphertext = xor_crypt(plaintext, key, nonce);
        let tag = compute_tag(associated_data, &ciphertext, key, nonce);

        let mut output = Vec::with_capacity(ciphertext.len() + 32);
        output.extend_from_slice(&ciphertext);
        output.extend_from_slice(&tag);

        Ok(output)
    }

    /// Decrypt an encrypted payload with a key, a nonce, and optional associated data.
    ///
    /// Verifies the 32-byte authentication tag before decrypting the ciphertext.
    pub fn decrypt(
        &self,
        ciphertext_and_tag: &[u8],
        key: &[u8],
        nonce: &[u8],
        associated_data: &[u8],
    ) -> Result<Vec<u8>, XaorError> {
        validate_key_and_nonce(key, nonce)?;

        if ciphertext_and_tag.len() < 32 {
            return Err(XaorError::DecryptionFailed);
        }

        let (ciphertext, tag) = ciphertext_and_tag.split_at(ciphertext_and_tag.len() - 32);
        let expected_tag = compute_tag(associated_data, ciphertext, key, nonce);

        if !constant_time_eq(&expected_tag, tag) {
            return Err(XaorError::DecryptionFailed);
        }

        let plaintext = xor_crypt(ciphertext, key, nonce);
        Ok(plaintext)
    }
}

fn validate_key_and_nonce(key: &[u8], nonce: &[u8]) -> Result<(), XaorError> {
    if key.len() < 16 {
        return Err(XaorError::InvalidConfig(
            "key must be at least 16 bytes".into(),
        ));
    }

    if nonce.len() < 12 {
        return Err(XaorError::InvalidConfig(
            "nonce must be at least 12 bytes".into(),
        ));
    }

    Ok(())
}

fn xor_crypt(input: &[u8], key: &[u8], nonce: &[u8]) -> Vec<u8> {
    let mut output = Vec::with_capacity(input.len());
    let mut counter = 0u64;
    let mut keystream = derive_keystream(key, nonce, counter);
    let mut stream_index = 0usize;

    for &byte in input {
        if stream_index >= keystream.len() {
            counter = counter.wrapping_add(1);
            keystream = derive_keystream(key, nonce, counter);
            stream_index = 0;
        }

        output.push(byte ^ keystream[stream_index]);
        stream_index += 1;
    }

    output
}

fn derive_keystream(key: &[u8], nonce: &[u8], counter: u64) -> Vec<u8> {
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"xaor.xcipher.keystream.v1");
    hasher.update(key);
    hasher.update(nonce);
    hasher.update(&counter.to_le_bytes());
    let mut stream = Vec::with_capacity(32);
    stream.extend_from_slice(hasher.finalize().as_bytes());
    stream
}

fn compute_tag(associated_data: &[u8], ciphertext: &[u8], key: &[u8], nonce: &[u8]) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"xaor.xcipher.mac.v1");
    hasher.update(key);
    hasher.update(nonce);
    hasher.update(associated_data);
    hasher.update(&(associated_data.len() as u64).to_le_bytes());
    hasher.update(ciphertext);
    hasher.update(&(ciphertext.len() as u64).to_le_bytes());
    *hasher.finalize().as_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_decrypt_roundtrip() {
        let engine = CipherEngine::new();
        let key = b"0123456789abcdef0123456789abcdef";
        let nonce = b"abcdefghijkl";
        let plaintext = b"Hello, secure world!";
        let ad = b"header-metadata";

        let ciphertext = engine.encrypt(plaintext, key, nonce, ad).unwrap();
        assert_ne!(ciphertext, plaintext);

        let decrypted = engine.decrypt(&ciphertext, key, nonce, ad).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn empty_plaintext_roundtrip() {
        let engine = CipherEngine::new();
        let key = b"0123456789abcdef0123456789abcdef";
        let nonce = b"abcdefghijkl";
        let plaintext = b"";
        let ad = b"";

        let ciphertext = engine.encrypt(plaintext, key, nonce, ad).unwrap();
        assert_eq!(ciphertext.len(), 32); // only tag

        let decrypted = engine.decrypt(&ciphertext, key, nonce, ad).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn invalid_key_length_is_rejected() {
        let engine = CipherEngine::new();
        let key = b"too-short-key";
        let nonce = b"abcdefghijkl";
        let plaintext = b"Hello";

        let err = engine.encrypt(plaintext, key, nonce, b"").unwrap_err();
        assert!(err.to_string().contains("key must be at least"));
    }

    #[test]
    fn invalid_nonce_length_is_rejected() {
        let engine = CipherEngine::new();
        let key = b"0123456789abcdef0123456789abcdef";
        let nonce = b"short";
        let plaintext = b"Hello";

        let err = engine.encrypt(plaintext, key, nonce, b"").unwrap_err();
        assert!(err.to_string().contains("nonce must be at least"));
    }

    #[test]
    fn tampered_ciphertext_fails_decryption() {
        let engine = CipherEngine::new();
        let key = b"0123456789abcdef0123456789abcdef";
        let nonce = b"abcdefghijkl";
        let plaintext = b"Hello, secure world!";
        let ad = b"metadata";

        let mut ciphertext = engine.encrypt(plaintext, key, nonce, ad).unwrap();
        // Tamper with the ciphertext body
        ciphertext[0] ^= 0x01;

        let err = engine.decrypt(&ciphertext, key, nonce, ad).unwrap_err();
        assert!(matches!(err, XaorError::DecryptionFailed));
    }

    #[test]
    fn tampered_associated_data_fails_decryption() {
        let engine = CipherEngine::new();
        let key = b"0123456789abcdef0123456789abcdef";
        let nonce = b"abcdefghijkl";
        let plaintext = b"Hello, secure world!";
        let ad = b"metadata";

        let ciphertext = engine.encrypt(plaintext, key, nonce, ad).unwrap();

        let err = engine.decrypt(&ciphertext, key, nonce, b"different-metadata").unwrap_err();
        assert!(matches!(err, XaorError::DecryptionFailed));
    }
}
