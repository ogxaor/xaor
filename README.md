# Xcrypt

## Adaptive Regenerative Cryptographic Engine

Version: 0.1 Draft
Status: Experimental Research Project

---

## Overview

Xcrypt is an experimental cryptographic research project aimed at designing a new category of adaptive security systems.

Unlike conventional cryptographic primitives that rely on fixed computation graphs and static transformation pipelines, Xcrypt introduces a dynamic architecture built around:

* entropy-aware state mutation
* compound computational hardness
* recursive cycle recycling
* topology-driven transformations
* memory-hard resistance

Xcrypt is not intended to immediately replace established cryptographic systems such as AES, SHA-3, or Argon2.

Instead, Xcrypt explores a new architectural layer that builds on trusted primitives while introducing adaptive structural complexity.

---

## Core Research Question

Can a cryptographic engine become significantly harder to attack if its internal topology evolves per input while remaining deterministic for legitimate verification and decryption?

---

## Primary Objectives

Xcrypt aims to achieve:

* high resistance to brute-force attacks
* high resistance to GPU/ASIC optimization
* strong diffusion and avalanche properties
* adaptive computational complexity
* support for both verification and encryption modes

---

## Core Engines

Xcrypt consists of six major engines:

### 1. Entropy Engine

Collects entropy from multiple domains.

### 2. Seed Engine

Produces deterministic master seeds.

### 3. Topology Engine

Generates computational graphs.

### 4. Compound Engine

Applies recursive hardening.

### 5. Cycle Recycler

Feeds transformed state back into later rounds.

### 6. Finalizer

Produces verifier or ciphertext.

---

## Experimental Subsystems

Xcrypt includes several experimental helper modules:

* **xnonce**: OsRng-based cryptographically secure nonce generator.
* **xtoken**: Cryptographically secure token generator with optional prefixes.
* **xid**: Context-aware deterministic ID generator with optional checksums.
* **xvault**: Local secret storage vault using master-key derived keystreams.
* **xproof**: Proof-of-work challenge issuer and verifier for anti-abuse rate-limiting.
* **xcipher**: Authenticated symmetric encryption (AEAD) using BLAKE3 as a stream cipher.

---

## Security Warning

Xcrypt is experimental.

It has not undergone:

* academic review
* formal cryptanalysis
* production audit
* adversarial benchmarking

Do not use Xcrypt in production environments until security validation is complete.

---

## CLI Usage

The repository now includes a built-in CLI for local use and automation:

```bash
xcrypt hash "mypassword"
xcrypt verify "mypassword" "$xcrypt$v=1$..."
xcrypt errors
xcrypt bench 5
xcrypt --profile interactive config
```

### Commands

* `hash <password>`: generate a stored Xcrypt hash
* `verify <password> <stored-hash>`: verify a password against a stored hash
* `errors`: print the built-in error catalog
* `bench [iterations]`: run a simple hashing benchmark
* `config`: print the active configuration

### Profiles

You can choose a preset with `--profile` or environment variables:

* `interactive`
* `standard`
* `high-security`
* `server`

Example environment variables:

```bash
XCRYPT_PROFILE=server
XCRYPT_ROUNDS=16
XCRYPT_MEMORY=256
XCRYPT_NODES=32
XCRYPT_OUTPUT_SIZE=64
XCRYPT_MODE=hash
```

### Notes

* `--profile` overrides `XCRYPT_PROFILE`
* `hash` and `verify` return errors with readable error codes when input is invalid
* `bench` uses the same configured pipeline as the library API
