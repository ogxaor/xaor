# Finalizer

---

# Introduction

The Finalizer is the terminal stage of Xcrypt.

Its responsibility is to convert the fully transformed internal state into a usable output artifact.

Depending on operating mode, this output may be:

* verifier digest
* ciphertext
* derived key
* keystream
* integrity tag

The Finalizer ensures all prior computation is compressed into a stable cryptographic output.

---

# Inputs

The Finalizer accepts:

* compound engine output
* recycler mutations
* memory arena final state
* metadata
* mode configuration

Represented as:

[
F_{in}
]

---

# Purpose

Why not directly use raw state?

Raw state has problems:

* large size
* irregular structure
* implementation complexity
* leakage risk

The Finalizer compresses state into canonical output.

---

# Operating Modes

Xcrypt supports two major modes.

---

# Mode 1 — Verification Mode

Used for password verification.

Example use cases:

* authentication
* password storage
* verifier generation

Output:

```text id="4c8g0m"
digest
```

Digest is irreversible.

Plain secret should never be recoverable.

---

# Mode 2 — Encryption Mode

Used for reversible encryption.

Example use cases:

* file encryption
* secret storage
* secure transport

Output:

```text id="f5x2at"
ciphertext
```

Ciphertext must be decryptable.

---

# Compression Function

Final compression:

[
O = H(F_{in})
]

Where H is secure compression primitive.

Recommended primitives:

* SHA3-512
* BLAKE3

Why?

Designing secure compression from scratch is extremely difficult.

---

# Digest Generation

Verification digest:

[
D = H(F_{in} || Salt || Config)
]

Properties required:

* collision resistance
* avalanche effect
* deterministic reproduction

Recommended output size:

```text id="zcw56v"
256–512 bits
```

---

# Keystream Generation

Encryption mode requires keystream generation.

Formula:

[
K = Expand(F_{in})
]

Expand converts final state into arbitrary-length pseudorandom stream.

---

# Encryption Operation

XOR-based stream encryption:

[
C = P \oplus K
]

Where:

* P = plaintext
* K = keystream
* C = ciphertext

---

# Decryption Operation

Because XOR is symmetric:

[
P = C \oplus K
]

Decryption requires identical keystream regeneration.

Thus:

* seed
* topology
* mutation
* memory evolution

must reproduce exactly.

---

# Integrity Protection

Encryption without integrity is dangerous.

Attackers may modify ciphertext.

Therefore Finalizer should generate integrity tag.

Example:

[
Tag = H(C || F_{in})
]

This detects tampering.

---

# Output Metadata

Final outputs should include metadata.

Example:

```text id="4s7w2j"
version
salt
rounds
memory_cost
mode
digest/ciphertext
tag
```

Metadata enables deterministic reconstruction.

---

# Failure Modes

---

## Weak Compression Primitive

Catastrophic.

---

## Metadata Loss

Decryption becomes impossible.

---

## Missing Integrity Validation

Ciphertext tampering may go undetected.

---

# Output

Finalizer produces:

* digest (verification mode)

or

* ciphertext + integrity tag (encryption mode)
