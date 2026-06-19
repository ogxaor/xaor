# Seed Engine

---

# Introduction

The Seed Engine converts:

* user input
* entropy vector
* configuration metadata

into a deterministic master seed.

This seed becomes the root of all later computation.

Everything downstream depends on seed quality.

---

# Purpose

Seed generation solves a critical problem.

Raw inputs are unsuitable for direct cryptographic processing.

Example raw inputs:

* password
* plaintext
* binary data

These vary in:

* length
* format
* entropy
* distribution

The Seed Engine standardizes them.

---

# Input Structure

Seed Engine accepts three major inputs.

---

## Secret Input

Examples:

* password
* plaintext
* key material

Represented as:

[
I
]

---

## Entropy Vector

From Entropy Engine.

Represented as:

[
E
]

---

## Configuration Metadata

Contains deterministic parameters.

Examples:

* version
* memory cost
* rounds
* graph settings

Represented as:

[
C
]

---

# Seed Formula

Master seed formula:

[
S_0 = H(I || E || C)
]

Where:

* H = secure compression function
* || = concatenation

Recommended hash primitives:

* BLAKE3
* SHA3-512

Why not invent our own compression?

Because secure compression is extremely difficult to design.

---

# Seed Properties

Master seed must satisfy:

---

## Property 1 — Deterministic

Same inputs produce same seed.

Required for:

* verification
* decryption

---

## Property 2 — Collision Resistant

Distinct inputs should not collide.

---

## Property 3 — Uniform Distribution

Seed bits should appear uniformly random.

---

## Property 4 — Avalanche Behavior

Small changes should dramatically change seed.

Example:

```text id="wbhmso"
password1
password2
```

1-character difference →

seed should change almost completely.

---

# Salt Integration

Salt prevents precomputation attacks.

Salt represented as:

[
Salt
]

Updated formula:

[
S_0=H(I||E||Salt||C)
]

Salt requirements:

* unique per input
* cryptographically random
* stored with output

Recommended salt size:

```text id="dgnj2l"
128 bits minimum
```

---

# Seed Expansion

Master seed may be expanded.

Why?

Later engines require multiple seeds.

Example:

* graph seed
* matrix seed
* memory seed
* round seed

Expansion:

```text id="l8cr6t"
master seed
    ↓
KDF expansion
    ↓
subseeds
```

---

# Subseed Derivation

Formula:

[
S_i = H(S_0 || i)
]

Example:

[
S_1
]

Graph seed

[
S_2
]

Memory seed

[
S_3
]

Round seed

---

# Seed Tree

ASCII diagram:

```text id="w5ux9j"
           Master Seed
          /    |    \
         /     |     \
    Graph   Memory   Round
```

This structure is called a seed tree.

---

# Failure Modes

---

## Weak Secret Input

Weak passwords remain vulnerable.

Algorithm cannot fully fix human weakness.

---

## Weak Salt

Repeated salts enable attack optimization.

---

## Broken Compression Primitive

Catastrophic.

---

# Output

Seed Engine output:

* master seed
* subseed tree

These become inputs to the Topology Engine.
