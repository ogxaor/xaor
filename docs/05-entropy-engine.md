# Entropy Engine

---

# Introduction

The Entropy Engine is the first operational stage of Xaor.

Its responsibility is to collect, normalize, evaluate, and mix entropy from multiple sources into a single structured entropy vector.

This entropy vector becomes the foundation for:

* seed generation
* topology mutation
* state diversification
* anti-optimization defenses

Without strong entropy, the entire engine weakens.

---

# Definition

Entropy refers to unpredictability.

In Xaor, entropy is not treated as a single number.

Instead, entropy is modeled as a multi-domain vector.

Definition:

[
E = [e_1, e_2, e_3, ..., e_n]
]

Where each component represents entropy from a distinct domain.

---

# Design Goals

The Entropy Engine must satisfy five requirements.

---

## Goal 1 — High Unpredictability

Generated entropy must be difficult to predict.

Low unpredictability reduces security.

---

## Goal 2 — Deterministic Reproduction (Optional)

Some entropy sources must be reproducible.

Example:

Password verification requires deterministic reconstruction.

Therefore entropy sources are classified into:

* reproducible
* non-reproducible

---

## Goal 3 — Independence

Entropy sources should avoid correlation.

Bad example:

```text id="3l7jtx"
timestamp
system_clock
clock_tick
```

These may correlate heavily.

Good entropy sources come from different domains.

---

## Goal 4 — Measurable Quality

Entropy quality must be evaluated.

Metrics:

* Shannon entropy
* min-entropy
* collision frequency
* repetition rate

---

## Goal 5 — Low Leakage

Entropy sources should not expose secret information.

---

# Entropy Source Categories

Xaor classifies entropy into four categories.

---

# Category 1 — Core Cryptographic Entropy

Mandatory.

Examples:

* secure RNG
* OS randomness pool
* hardware RNG

Examples of OS sources:

### Linux

```text id="92awjd"
/dev/random
/dev/urandom
```

### Windows

Cryptographic API randomness.

This is primary entropy.

---

# Category 2 — System Entropy

Derived from system behavior.

Examples:

* process jitter
* scheduler timing
* CPU cycle variance
* thread timing

These sources exploit nondeterministic scheduling behavior.

---

# Category 3 — Memory Entropy

Derived from memory behavior.

Examples:

* allocation timing
* cache variance
* page behavior

Useful for subtle diversification.

---

# Category 4 — Experimental External Entropy

Optional.

Examples:

* weather data
* news feeds
* blockchain block hashes
* external APIs

Important:

External entropy is NOT secret.

It only contributes diversity.

Never rely on public entropy as sole security source.

---

# Entropy Vector Structure

Example vector:

```text id="6otj5h"
E = [
  core_rng,
  cpu_jitter,
  memory_jitter,
  external_seed
]
```

Each element normalized to fixed width.

Recommended width:

```text id="r28bso"
64 bits per component
```

Example:

4 components →

256-bit entropy vector.

---

# Entropy Normalization

Raw entropy sources vary in format.

Examples:

* integers
* timestamps
* byte streams
* API strings

Normalization converts all sources into fixed-size binary representation.

Example pipeline:

```text id="s2iy06"
Raw Input
   ↓
Encode
   ↓
Normalize
   ↓
Entropy Block
```

---

# Entropy Mixing

Entropy blocks are mixed into one structure.

Mixing function:

[
E_{mixed}=Mix(e_1,e_2,...,e_n)
]

Requirements:

Mix must provide:

* diffusion
* low bias
* uniform spread

Possible primitives:

* XOR
* modular addition
* hash compression

Recommended approach:

```text id="mqrxha"
concat(all sources)
      ↓
BLAKE3 / SHA3-512
      ↓
mixed entropy
```

---

# Entropy Quality Estimation

Poor entropy must be detected.

Metrics:

---

## Shannon Entropy

Measures average uncertainty.

---

## Min-Entropy

Worst-case predictability.

Formula:

[
H_{\infty}(X)= -\log_2(max(P(x)))
]

Very important for security.

---

## Collision Rate

High collisions indicate poor entropy diversity.

---

# Entropy Failure Modes

---

## Failure 1 — Predictable RNG

Catastrophic.

Weak RNG destroys cryptographic security.

---

## Failure 2 — Correlated Sources

Multiple sources may appear diverse but are mathematically related.

---

## Failure 3 — Public Entropy Dependence

Weather or news APIs must never be primary entropy.

---

# Output

Entropy Engine output:

[
E_{final}
]

Recommended size:

```text id="cbq7xg"
256–512 bits
```

This output is passed into the Seed Engine.
