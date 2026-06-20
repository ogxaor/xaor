//! Experimental symmetric cipher providing authenticated encryption (AEAD).
//!
//! ## Construction
//!
//! `CipherEngine` implements an **AEAD** (Authenticated Encryption with
//! Associated Data) scheme built from two BLAKE3-based primitives:
//!
//! ### Encryption: Custom CTR Mode
//! ```text
//! keystream_block[i] = BLAKE3("xaor.xcipher.keystream.v1" || key || nonce || LE64(i))
//! ciphertext         = plaintext XOR keystream
//! ```
//! Each 32-byte keystream block is derived from BLAKE3 keyed with the secret.
//! The counter `i` advances every 32 bytes of plaintext. This is a standard
//! counter-mode stream cipher construction — **not** a raw XOR cipher.
//!
//! ### Authentication: Encrypt-then-MAC (EtM)
//! ```text
//! tag = BLAKE3("xaor.xcipher.mac.v1" || key || nonce || AD || len(AD) || ciphertext || len(ciphertext))
//! ```
//! The tag covers **both** the associated data and the ciphertext, satisfying
//! AEAD compliance. Decryption verifies the tag before touching the ciphertext
//! (Chosen-Ciphertext Attack resistance).
//!
//! ### Key Size
//! Minimum 32 bytes (256 bits). BLAKE3 provides 256-bit security internally;
//! feeding it less than 32 bytes of key material would under-utilize its
//! security margin.

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
    /// Returns `ciphertext || 32-byte MAC tag`.
    ///
    /// # Key Policy
    /// `key` must be at least 32 bytes. Keys shorter than 32 bytes provide less
    /// than 256-bit security and are rejected.
    ///
    /// # Nonce Policy
    /// `nonce` must be at least 12 bytes. **Never reuse a nonce with the same key.**
    /// Use `NonceEngine` to generate cryptographically random nonces.
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
    /// Verifies the 32-byte MAC tag **before** decrypting. Any modification to
    /// the ciphertext, associated data, key, or nonce causes immediate rejection
    /// with `DecryptionFailed` — preventing Padding Oracle / Chosen-Ciphertext
    /// attacks.
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

        // TIMING: constant_time_eq is backed by `subtle::ConstantTimeEq`, which
        // uses a compiler memory barrier to prevent LLVM from short-circuiting or
        // vectorizing the comparison. The MAC check cannot leak information about
        // where (or if) the tag differs.
        if !constant_time_eq(&expected_tag, tag) {
            return Err(XaorError::DecryptionFailed);
        }

        let plaintext = xor_crypt(ciphertext, key, nonce);
        Ok(plaintext)
    }
}

/// Validate that `key` is at least 32 bytes and `nonce` is at least 12 bytes.
///
/// 32 bytes = 256-bit key material, matching BLAKE3's full security margin.
fn validate_key_and_nonce(key: &[u8], nonce: &[u8]) -> Result<(), XaorError> {
    if key.len() < 32 {
        return Err(XaorError::InvalidConfig(
            "key must be at least 32 bytes (256 bits)".into(),
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

/// Compute the 32-byte AEAD authentication tag.
///
/// The tag covers: domain separator, key, nonce, associated data AND its length,
/// ciphertext AND its length. Length-prefixing the AD and ciphertext prevents
/// length-extension / canonicalization attacks where two different
/// (AD, ciphertext) pairs produce the same concatenation.
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
    use rand::rngs::OsRng;
    use rand::RngCore;

    fn make_key() -> [u8; 32] {
        *b"0123456789abcdef0123456789abcdef"
    }

    fn make_nonce() -> [u8; 12] {
        *b"abcdefghijkl"
    }

    // ── Correctness ───────────────────────────────────────────────────────────

    #[test]
    fn encrypt_decrypt_roundtrip() {
        let engine = CipherEngine::new();
        let plaintext = b"Hello, secure world!";
        let ad = b"header-metadata";

        let ciphertext = engine.encrypt(plaintext, &make_key(), &make_nonce(), ad).unwrap();
        assert_ne!(&ciphertext[..plaintext.len()], plaintext.as_slice());

        let decrypted = engine.decrypt(&ciphertext, &make_key(), &make_nonce(), ad).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn empty_plaintext_roundtrip() {
        let engine = CipherEngine::new();
        let ad = b"";

        let ciphertext = engine.encrypt(b"", &make_key(), &make_nonce(), ad).unwrap();
        assert_eq!(ciphertext.len(), 32); // only the MAC tag

        let decrypted = engine.decrypt(&ciphertext, &make_key(), &make_nonce(), ad).unwrap();
        assert_eq!(decrypted, b"");
    }

    // ── Input validation ──────────────────────────────────────────────────────

    #[test]
    fn short_key_is_rejected() {
        let engine = CipherEngine::new();
        let short_key = b"too-short-key!!!"; // 16 bytes — below 32-byte minimum
        let err = engine.encrypt(b"Hello", short_key, &make_nonce(), b"").unwrap_err();
        assert!(err.to_string().contains("32 bytes"), "expected 32-byte error, got: {err}");
    }

    #[test]
    fn invalid_nonce_length_is_rejected() {
        let engine = CipherEngine::new();
        let err = engine.encrypt(b"Hello", &make_key(), b"short", b"").unwrap_err();
        assert!(err.to_string().contains("nonce must be at least"));
    }

    // ── Adversarial Benchmark 3 (Chosen-Ciphertext / MAC) ────────────────────

    #[test]
    fn tampered_ciphertext_fails_decryption() {
        let engine = CipherEngine::new();
        let plaintext = b"Hello, secure world!";
        let ad = b"metadata";

        let mut ciphertext = engine.encrypt(plaintext, &make_key(), &make_nonce(), ad).unwrap();
        ciphertext[0] ^= 0x01; // flip 1 bit in the ciphertext body

        let err = engine.decrypt(&ciphertext, &make_key(), &make_nonce(), ad).unwrap_err();
        assert!(matches!(err, XaorError::DecryptionFailed),
            "tampered ciphertext must be rejected, got: {err}");
    }

    #[test]
    fn tampered_associated_data_fails_decryption() {
        let engine = CipherEngine::new();
        let plaintext = b"Hello, secure world!";
        let ad = b"metadata";

        let ciphertext = engine.encrypt(plaintext, &make_key(), &make_nonce(), ad).unwrap();
        let err = engine
            .decrypt(&ciphertext, &make_key(), &make_nonce(), b"different-metadata")
            .unwrap_err();
        assert!(matches!(err, XaorError::DecryptionFailed));
    }

    #[test]
    fn tampered_mac_tag_fails_decryption() {
        let engine = CipherEngine::new();
        let plaintext = b"sensitive data";
        let ad = b"";

        let mut ct = engine.encrypt(plaintext, &make_key(), &make_nonce(), ad).unwrap();
        // Flip a bit in the last 32-byte MAC tag
        let tag_start = ct.len() - 32;
        ct[tag_start] ^= 0xFF;

        let err = engine.decrypt(&ct, &make_key(), &make_nonce(), ad).unwrap_err();
        assert!(matches!(err, XaorError::DecryptionFailed),
            "tampered MAC must be rejected immediately");
    }

    // ── Adversarial Benchmark 1 (Avalanche Effect) ────────────────────────────
    //
    // A 1-bit flip in the key must cause approximately 50% of the ciphertext
    // bits to flip. We assert ≥ 45% to allow for statistical variance.

    #[test]
    fn avalanche_effect_one_bit_key_flip() {
        let engine = CipherEngine::new();
        let plaintext = b"The quick brown fox jumps over the lazy dog -- avalanche test payload";
        let ad = b"ad";

        let ct1 = engine.encrypt(plaintext, &make_key(), &make_nonce(), ad).unwrap();

        // Flip exactly 1 bit in the key
        let mut flipped_key = make_key();
        flipped_key[0] ^= 0x01;
        let ct2 = engine.encrypt(plaintext, &flipped_key, &make_nonce(), ad).unwrap();

        // Only compare the ciphertext body (not the MAC, which is allowed to
        // differ in its entirety since the key changed)
        let body_len = plaintext.len();
        let flipped_bits: u32 = ct1[..body_len]
            .iter()
            .zip(ct2[..body_len].iter())
            .map(|(a, b)| (a ^ b).count_ones())
            .sum();
        let total_bits = (body_len * 8) as f64;
        let flip_rate = flipped_bits as f64 / total_bits;

        assert!(
            flip_rate >= 0.45,
            "Avalanche FAILED: only {:.1}% of ciphertext bits changed after 1-bit key flip \
             (expected ≥ 45%). The keystream diffusion is too weak.",
            flip_rate * 100.0
        );
    }

    // ── Adversarial Benchmark 2 (Replay / Nonce Uniqueness) ──────────────────
    //
    // Encrypting the same plaintext twice with different random nonces MUST
    // produce different ciphertexts. If nonces are reused, an XOR of the two
    // ciphertexts reveals the XOR of the two plaintexts.

    #[test]
    fn replay_resistance_different_nonces_produce_different_ciphertexts() {
        let engine = CipherEngine::new();
        let plaintext = b"attack at dawn";
        let ad = b"";

        let mut nonce1 = [0u8; 16];
        let mut nonce2 = [0u8; 16];
        OsRng.fill_bytes(&mut nonce1);
        OsRng.fill_bytes(&mut nonce2);

        // In the astronomically unlikely event OsRng returns the same nonce twice
        // (probability 2^-128), regenerate.
        assert_ne!(nonce1, nonce2, "OsRng returned identical 128-bit nonces — RNG is broken");

        let ct1 = engine.encrypt(plaintext, &make_key(), &nonce1, ad).unwrap();
        let ct2 = engine.encrypt(plaintext, &make_key(), &nonce2, ad).unwrap();

        assert_ne!(ct1, ct2,
            "Replay FAILED: same plaintext encrypted with two different nonces produced \
             identical ciphertexts. Nonce is not being incorporated into the keystream.");
    }
}

