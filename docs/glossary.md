# Glossary

---

## Arena

Large memory region used to impose memory-hard computation.

---

## Avalanche Effect

Property where tiny input changes cause large output changes.

Ideal cryptographic systems approach ~50% output bit changes.

---

## Collision

Two distinct inputs producing same output.

---

## Compound Engine

Xcrypt engine responsible for recursive hardening across rounds.

---

## Compression Function

Function that reduces large internal state into fixed-size output.

---

## Cycle Recycler

Subsystem that recycles round output into future computation.

---

## Deterministic

Same input always produces same output.

Required for verification and decryption.

---

## Digest

Final irreversible verification output.

Used for password checking.

---

## Entropy

Measure of unpredictability.

Higher entropy usually implies stronger resistance to guessing.

---

## Finalizer

Terminal engine that produces digest, keystream, or ciphertext.

---

## Graph Topology

Structure describing node connections and traversal order.

---

## Integrity Tag

Cryptographic tag used to detect ciphertext modification.

---

## Keystream

Pseudorandom byte stream used for stream encryption.

---

## Memory Hardness

Property that increases required RAM and memory bandwidth.

---

## Mutation

Structural modification of future computation based on prior state.

---

## Salt

Random value added to inputs to prevent precomputation attacks.

---

## Seed

Deterministic master state generated from input and entropy.

---

## State

Internal mutable data processed by Xcrypt engines.

---

## Topology Engine

Engine responsible for computational graph generation.

---

## Verification Mode

Mode used for password hashing / digest comparison.
