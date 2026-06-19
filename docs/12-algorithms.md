# Core Algorithms

---

# Overview

This chapter defines the end-to-end algorithms used by Xcrypt.

Three core algorithms exist:

1. Verification
2. Encryption
3. Decryption

---

# Algorithm 1 — Verification

Used for password storage.

---

## Registration

User secret:

```text id="m3d1ok"
password
```

Pipeline:

```text id="qjlwmv"
password
   ↓
Entropy Engine
   ↓
Seed Engine
   ↓
Topology Engine
   ↓
Compound Engine
   ↓
Cycle Recycler
   ↓
Memory Arena
   ↓
Finalizer
   ↓
digest
```

Store:

* digest
* salt
* metadata

---

## Login Verification

User input:

```text id="d5ylm8"
candidate password
```

Pipeline repeated with same metadata.

Compare digests:

[
digest_1 == digest_2
]

If equal:

Authentication succeeds.

---

# Verification Pseudocode

```rust
fn verify(secret, stored_digest, metadata) -> bool {
    entropy = entropy_engine(metadata);
    seed = seed_engine(secret, entropy, metadata);
    graph = topology_engine(seed);
    state = compound_engine(seed, graph);
    state = recycle(state);
    state = memory_arena(state);
    digest = finalize_verify(state, metadata);

    constant_time_compare(digest, stored_digest)
}
```

---

# Algorithm 2 — Encryption

Used for reversible protection.

---

## Encryption Flow

Input:

```text id="i9ep2s"
plaintext
```

Pipeline:

```text id="r7i0au"
plaintext
    ↓
entropy engine
    ↓
seed engine
    ↓
topology engine
    ↓
compound engine
    ↓
recycler
    ↓
memory arena
    ↓
finalizer
    ↓
keystream
```

Encrypt:

[
ciphertext = plaintext \oplus keystream
]

---

# Encryption Pseudocode

```rust
fn encrypt(plaintext, key, metadata) -> Cipher {
    entropy = entropy_engine(metadata);
    seed = seed_engine(key, entropy, metadata);
    graph = topology_engine(seed);
    state = compound_engine(seed, graph);
    state = recycle(state);
    state = memory_arena(state);

    keystream = finalize_encrypt(state, plaintext.len());
    ciphertext = xor(plaintext, keystream);
    tag = integrity_tag(ciphertext, state);

    Cipher { ciphertext, tag }
}
```

---

# Algorithm 3 — Decryption

Input:

* ciphertext
* key
* metadata

Regenerate identical keystream.

---

# Decryption Flow

```text id="c8jqg6"
ciphertext
     ↓
same pipeline
     ↓
keystream
     ↓
XOR
     ↓
plaintext
```

---

# Decryption Pseudocode

```rust
fn decrypt(cipher, key, metadata) -> Plaintext {
    entropy = entropy_engine(metadata);
    seed = seed_engine(key, entropy, metadata);
    graph = topology_engine(seed);
    state = compound_engine(seed, graph);
    state = recycle(state);
    state = memory_arena(state);

    verify_tag(cipher.tag, cipher.ciphertext, state);

    keystream = finalize_encrypt(state, cipher.len());
    plaintext = xor(cipher.ciphertext, keystream);

    plaintext
}
```

---

# Complexity

Approximate runtime:

[
O(E + S + G + R + M)
]

Where:

* E = entropy cost
* S = seed cost
* G = graph generation
* R = rounds
* M = memory operations

---

# Summary

Xcrypt algorithms combine all prior engines into deterministic end-to-end pipelines.
