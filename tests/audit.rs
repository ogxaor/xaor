//! # Xaor Cryptographic Audit Test Suite
//!
//! Integration tests covering the "Killer Audit Framework" adversarial benchmarks:
//!
//! 1. **Integrity**: Round-trip encrypt/decrypt must be identity.
//! 2. **Avalanche Effect**: 1-bit key/ciphertext flip → ~50% output bit change.
//! 3. **Replay Resistance**: Same plaintext + different nonces → different ciphertexts.
//! 4. **Chosen-Ciphertext (MAC)**: Any modification to ciphertext/AD → immediate rejection.
//! 5. **Password Hash Uniqueness**: Same password → different stored hash (random salt).
//! 6. **Verify Round-Trip**: hash_password → verify_password must succeed.
//! 7. **Constant-Time Contract**: constant_time_eq must handle all lengths safely.

use xaor::{CipherEngine, Xaor};
use xaor::constant_time_eq;
use rand::rngs::OsRng;
use rand::RngCore;

/// A fixed 32-byte test key (256-bit).
fn test_key() -> [u8; 32] {
    *b"xaor-audit-key-32bytes-v1-000000"
}

/// A fixed 16-byte test nonce (128-bit).
fn test_nonce() -> [u8; 16] {
    *b"xaor-audit-nonce"
}

// ── 1. Integrity: Round-Trip ───────────────────────────────────────────────────

#[test]
fn integrity_encrypt_decrypt_roundtrip_is_identity() {
    let engine = CipherEngine::new();
    let plaintext = b"Xaor round-trip integrity check: this must decrypt to exactly these bytes.";
    let ad = b"xaor-audit-associated-data-v1";

    let ciphertext = engine
        .encrypt(plaintext, &test_key(), &test_nonce(), ad)
        .expect("encryption must not fail with valid inputs");

    let recovered = engine
        .decrypt(&ciphertext, &test_key(), &test_nonce(), ad)
        .expect("decryption of own output must not fail");

    assert_eq!(
        recovered, plaintext,
        "INTEGRITY FAILED: decrypted output does not match original plaintext"
    );
}

#[test]
fn integrity_empty_plaintext_roundtrip() {
    let engine = CipherEngine::new();
    let ct = engine.encrypt(b"", &test_key(), &test_nonce(), b"ad").unwrap();
    let pt = engine.decrypt(&ct, &test_key(), &test_nonce(), b"ad").unwrap();
    assert_eq!(pt, b"", "empty plaintext round-trip failed");
}

#[test]
fn integrity_large_plaintext_roundtrip() {
    let engine = CipherEngine::new();
    let plaintext: Vec<u8> = (0u8..=255).cycle().take(4096).collect();
    let ad = b"large-payload-test";

    let ct = engine.encrypt(&plaintext, &test_key(), &test_nonce(), ad).unwrap();
    let pt = engine.decrypt(&ct, &test_key(), &test_nonce(), ad).unwrap();

    assert_eq!(pt, plaintext, "4096-byte round-trip failed");
}

// ── 2. Adversarial Benchmark 1: Avalanche Effect ──────────────────────────────
//
// Per the audit framework: "Change 1 bit of your key. Does the decrypted output
// become complete, random garbage?" We measure on the CIPHERTEXT, not decrypted
// output (since decryption of a tampered ciphertext is properly rejected).

#[test]
fn adversarial_avalanche_one_bit_key_flip_causes_wide_ciphertext_diffusion() {
    let engine = CipherEngine::new();
    // Long enough for a statistically meaningful bit-flip ratio
    let plaintext: Vec<u8> = (0u8..128).collect();
    let ad = b"avalanche-test";

    let ct_original = engine
        .encrypt(&plaintext, &test_key(), &test_nonce(), ad)
        .unwrap();

    let mut flipped_key = test_key();
    flipped_key[15] ^= 0x80; // flip the MSB of byte 15

    let ct_flipped = engine
        .encrypt(&plaintext, &flipped_key, &test_nonce(), ad)
        .unwrap();

    let body_len = plaintext.len();
    let flipped_bits: u32 = ct_original[..body_len]
        .iter()
        .zip(ct_flipped[..body_len].iter())
        .map(|(a, b)| (a ^ b).count_ones())
        .sum();
    let total_bits = (body_len * 8) as f64;
    let flip_rate = flipped_bits as f64 / total_bits;

    assert!(
        flip_rate >= 0.45,
        "AVALANCHE FAILED: only {:.1}% of ciphertext body bits changed after 1-bit key flip. \
         Expected ≥ 45%. This suggests the keystream function has poor diffusion.",
        flip_rate * 100.0
    );
}

#[test]
fn adversarial_avalanche_one_bit_plaintext_change_changes_mac() {
    // Changing 1 bit of plaintext must produce a completely different MAC tag.
    let engine = CipherEngine::new();
    let mut plaintext = b"constant plaintext for mac diffusion test".to_vec();

    let ct1 = engine.encrypt(&plaintext, &test_key(), &test_nonce(), b"").unwrap();

    plaintext[0] ^= 0x01;
    let ct2 = engine.encrypt(&plaintext, &test_key(), &test_nonce(), b"").unwrap();

    // Both ciphertexts have 32-byte MAC tags at the end
    let mac1 = &ct1[ct1.len() - 32..];
    let mac2 = &ct2[ct2.len() - 32..];
    assert_ne!(mac1, mac2, "1-bit plaintext change must produce a different MAC tag");
}

// ── 3. Adversarial Benchmark 2: Replay Resistance ─────────────────────────────
//
// "Encrypt the same message twice. Are the ciphertexts different?"

#[test]
fn adversarial_replay_same_plaintext_different_nonces_yields_different_ciphertexts() {
    let engine = CipherEngine::new();
    let plaintext = b"identical message encrypted twice";
    let ad = b"";

    let mut nonce_a = [0u8; 16];
    let mut nonce_b = [0u8; 16];
    OsRng.fill_bytes(&mut nonce_a);
    OsRng.fill_bytes(&mut nonce_b);
    // Guarantee nonces differ (OsRng collision probability ≈ 2^-128)
    assert_ne!(nonce_a, nonce_b);

    let ct_a = engine.encrypt(plaintext, &test_key(), &nonce_a, ad).unwrap();
    let ct_b = engine.encrypt(plaintext, &test_key(), &nonce_b, ad).unwrap();

    assert_ne!(
        ct_a, ct_b,
        "REPLAY FAILED: same plaintext with two distinct nonces produced identical ciphertexts. \
         The nonce is not being incorporated into the keystream derivation."
    );
}

#[test]
fn adversarial_replay_nonce_reuse_produces_identical_ciphertext() {
    // Inverse test: same key + same nonce → same ciphertext (deterministic).
    // This documents the EXPECTED behavior (and why nonce reuse is dangerous).
    let engine = CipherEngine::new();
    let plaintext = b"deterministic output with fixed nonce";

    let ct1 = engine.encrypt(plaintext, &test_key(), &test_nonce(), b"").unwrap();
    let ct2 = engine.encrypt(plaintext, &test_key(), &test_nonce(), b"").unwrap();

    assert_eq!(
        ct1, ct2,
        "Determinism check: same key+nonce+plaintext must produce same ciphertext"
    );
}

// ── 4. Adversarial Benchmark 3: Chosen-Ciphertext / MAC ──────────────────────
//
// "If you feed the cipher modified data, does it immediately reject it via MAC?"

#[test]
fn adversarial_cca_single_bit_flip_in_ciphertext_body_is_rejected() {
    let engine = CipherEngine::new();
    let plaintext = b"authenticated encryption test payload -- must not decrypt if tampered";
    let ad = b"audit-cca-test";

    let mut ct = engine.encrypt(plaintext, &test_key(), &test_nonce(), ad).unwrap();
    ct[0] ^= 0x01; // 1-bit flip in byte 0 of the ciphertext body

    let result = engine.decrypt(&ct, &test_key(), &test_nonce(), ad);
    assert!(
        result.is_err(),
        "CCA FAILED: ciphertext with 1-bit flip was NOT rejected. \
         Decryption must fail before touching the plaintext."
    );
}

#[test]
fn adversarial_cca_flip_last_bit_of_mac_tag_is_rejected() {
    let engine = CipherEngine::new();
    let plaintext = b"mac integrity check";
    let ad = b"";

    let mut ct = engine.encrypt(plaintext, &test_key(), &test_nonce(), ad).unwrap();
    let last = ct.len() - 1;
    ct[last] ^= 0x01; // flip 1 bit in the very last byte of the MAC tag

    let result = engine.decrypt(&ct, &test_key(), &test_nonce(), ad);
    assert!(result.is_err(), "MAC tag modification was not rejected");
}

#[test]
fn adversarial_cca_modified_associated_data_is_rejected() {
    let engine = CipherEngine::new();
    let plaintext = b"payload";
    let ad = b"original-header";

    let ct = engine.encrypt(plaintext, &test_key(), &test_nonce(), ad).unwrap();

    // Attempt to decrypt with tampered AD
    let result = engine.decrypt(&ct, &test_key(), &test_nonce(), b"tampered-header");
    assert!(result.is_err(), "Tampered associated data was not rejected by MAC");
}

#[test]
fn adversarial_cca_truncated_ciphertext_is_rejected() {
    let engine = CipherEngine::new();
    let plaintext = b"truncation attack test";

    let ct = engine.encrypt(plaintext, &test_key(), &test_nonce(), b"").unwrap();
    // Truncate the ciphertext so the MAC tag is partially missing
    let truncated = &ct[..ct.len() - 1];

    let result = engine.decrypt(truncated, &test_key(), &test_nonce(), b"");
    assert!(result.is_err(), "Truncated ciphertext must be rejected");
}

// ── 5. Password Hash Uniqueness ────────────────────────────────────────────────

#[test]
fn password_same_input_produces_different_stored_hashes() {
    let xaor = Xaor::default();
    let password = "hunter2";

    let hash1 = xaor.hash_password(password).expect("hash1 must succeed");
    let hash2 = xaor.hash_password(password).expect("hash2 must succeed");

    assert_ne!(
        hash1, hash2,
        "PASSWORD UNIQUENESS FAILED: hashing the same password twice produced identical \
         stored hashes. The salt is not being randomized per call."
    );
}

// ── 6. Password Verify Round-Trip ─────────────────────────────────────────────

#[test]
fn password_verify_roundtrip_succeeds() {
    let xaor = Xaor::default();
    let password = "correct-horse-battery-staple";

    let stored = xaor.hash_password(password).expect("hashing must succeed");
    let ok = xaor
        .verify_password(password, &stored)
        .expect("verification must not error");

    assert!(ok, "VERIFY ROUND-TRIP FAILED: correct password was rejected");
}

#[test]
fn password_verify_wrong_password_fails() {
    let xaor = Xaor::default();
    let stored = xaor.hash_password("correct-password").unwrap();
    let ok = xaor.verify_password("wrong-password", &stored).unwrap();
    assert!(!ok, "Wrong password must not verify as correct");
}

// ── 7. Constant-Time Contract ─────────────────────────────────────────────────

#[test]
fn constant_time_eq_equal_slices_returns_true() {
    let a = b"xaor-constant-time-test-32bytes!";
    let b = b"xaor-constant-time-test-32bytes!";
    assert!(constant_time_eq(a, b));
}

#[test]
fn constant_time_eq_unequal_slices_returns_false() {
    let a = b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    let b = b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaB"; // last byte differs
    assert!(!constant_time_eq(a, b));
}

#[test]
fn constant_time_eq_different_lengths_returns_false() {
    assert!(!constant_time_eq(b"short", b"longer slice"));
}

#[test]
fn constant_time_eq_empty_slices_are_equal() {
    assert!(constant_time_eq(b"", b""));
}
