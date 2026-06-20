# Subsystems Reference

---

## Core Error Catalog

All modules in Xaor propagate errors through the unified `XaorError` type. Each error variant maps to a distinct, uppercase error code:

| Error Code String | Description | Underlying Root Cause |
| :--- | :--- | :--- |
| **`ENTROPY_ERROR`** | Entropy generation failed | System RNG failed to collect sufficient byte entropy |
| **`SEED_ERROR`** | Seed generation failed | BLAKE3 domain-separated mixing failed |
| **`TOPOLOGY_ERROR`** | Topology generation failed | Matrix or traversal path layout contains errors |
| **`COMPOUND_ERROR`** | Compound engine failed | Subkey evaluation overflow or mixing failure |
| **`RECYCLER_ERROR`** | Recycler stage failed | State recycle buffers could not register |
| **`MEMORY_ERROR`** | Memory arena allocation failed | Virtual memory exhaustion or pass initialization error |
| **`FINALIZER_ERROR`** | Finalizer failed | SHA3/BLAKE3-XOF digest output generation failed |
| **`INVALID_CONFIG`** | Configuration validation failed | Out of bounds memory (0MB) or rounds (0) parameters |
| **`VERIFICATION_FAILED`**| Password verification failed | Input digest mismatch during constant-time comparison |
| **`ENCRYPTION_FAILED`** | Authenticated encryption failed | Ciphertext generation or AEAD mixing failed |
| **`DECRYPTION_FAILED`** | Authenticated decryption failed | Integrity validation failed or ciphertext modified |
| **`MALFORMED_STORED_HASH`**| Stored hash is malformed | String parser failed to split or read parameters |
| **`UNSUPPORTED_STORED_HASH_VERSION`**| Unsupported version | Hash protocol version mismatch |

---

## 1. Nonce Engine (`xnonce`)

Generates cryptographically secure, random nonce values.

### Rust API
```rust
use xaor::xnonce::NonceEngine;

// Create engine specifying desired bytes length
let engine = NonceEngine::new(32)?;

// Generate hexadecimal string representation
let hex_nonce = engine.generate_hex()?; // 64 character hex string
```

---

## 2. Token Engine (`xtoken`)

Generates secure random tokens with optional custom prefixes.

### Rust API
```rust
use xaor::xtoken::TokenEngine;

// Create engine with length 32 and prefix "tk"
let engine = TokenEngine::with_prefix(32, Some("tk"))?;

// Generates e.g. "tk_ab38de..."
let token = engine.generate()?;
```

---

## 3. Context ID Engine (`xid`)

Generates context-aware, URL-safe identifiers containing optional prefixes and mathematical parity checks (checksums).

### Configuration Struct (`XidConfig`)
* `length: usize`: Total length of the generated ID (excluding prefix). Must be between 1 and 128.
* `alphabet: String`: Custom character set. Must be ASCII, contain at least 2 characters, and have no duplicates.
* `prefix: Option<String>`: Custom string prepended to the ID.
* `checksum: bool`: Enable math parity validations.

### Rust API
```rust
use xaor::xid::{XidConfig, XidEngine};

// 1. Build configuration
let config = XidConfig::with_options(
    24, 
    "0123456789abcdef".to_string(), // Hexadecimal alphabet
    Some("user".to_string()),        // Prefix
    true                             // Enable parity checksum
)?;

// 2. Instantiate engine
let engine = XidEngine::new(config)?;

// 3. Generate ID (e.g., "user_74df83c84ea819bc2e18d40a")
let id = engine.generate()?;

// 4. Validate ID
let is_valid = engine.validate(&id)?;
assert!(is_valid);
```

---

## 4. Cipher Engine (`xcipher`)

Provides symmetric, authenticated encryption (AEAD) based on custom BLAKE3 diffusion mixing.

### Rust API
```rust
use xaor::xcipher::CipherEngine;

let engine = CipherEngine::new();

let plaintext = b"sensitive payload data";
let key = b"0123456789abcdef0123456789abcdef"; // 32-byte key
let nonce = b"0123456789abcdef";               // 16-byte nonce
let associated_data = b"metadata context";

// 1. Encrypt
let ciphertext = engine.encrypt(plaintext, key, nonce, associated_data)?;

// 2. Decrypt
let decrypted = engine.decrypt(&ciphertext, key, nonce, associated_data)?;
assert_eq!(plaintext.to_vec(), decrypted);
```

---

## 5. Vault Engine (`xvault`)

Encapsulates secret key storage, writing encrypted payloads directly to local files.

### Rust API
```rust
use xaor::xvault::VaultEngine;

let master_key = b"super-secret-vault-master-key-32b";
let engine = VaultEngine::new("./my_vault.db");

// 1. Store secret under key "api_key"
let secret_data = b"sk_live_51...";
engine.store("api_key", secret_data, master_key)?;

// 2. Retrieve secret
if let Some(decrypted_secret) = engine.retrieve("api_key", master_key)? {
    assert_eq!(decrypted_secret, secret_data.to_vec());
}
```

---

## 6. Proof Engine (`xproof`)

Implements a CPU-hard Proof-of-Work challenge/solve protocol.

### Configuration Struct (`ProofConfig`)
* `difficulty_bits: u32`: Target zero-bit prefix count. Higher difficulty values require longer computations.
* `ttl_secs: u64`: Challenge lifespan in seconds.

### Rust API
```rust
use xaor::xproof::{ProofConfig, ProofEngine};

let config = ProofConfig {
    difficulty_bits: 8,
    ttl_secs: 300,
};

let engine = ProofEngine::new(config)?;

// 1. Create challenge for subject
let challenge = engine.challenge("client-ip-address")?;
let serialized_challenge = challenge.serialize();

// 2. Client solves challenge
let client_engine = ProofEngine::default_low();
let solution = client_engine.solve(&challenge)?;
let serialized_solution = solution.serialize();

// 3. Server verifies solution
let is_valid = engine.verify(&challenge, &solution)?;
assert!(is_valid);
```
