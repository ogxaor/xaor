# Threat Model

---

# Why Threat Modeling Matters

Cryptography without a threat model is incomplete.

Security claims only have meaning relative to attackers.

We must define:

* who attacks
* what resources they have
* what capabilities they possess
* what Xaor can and cannot defend against

---

# Attacker Categories

---

# Level 1 — Casual Attacker

Resources:

* personal computer
* public cracking tools
* limited technical knowledge

Capabilities:

* brute force
* credential stuffing
* dictionary attacks

Risk:
Low to moderate.

Xaor should strongly resist this category.

---

# Level 2 — Professional Adversary

Resources:

* multiple GPUs
* optimized cracking rigs
* specialized tooling

Capabilities:

* distributed brute force
* large-scale parallel cracking
* memory optimization attacks

Risk:
High.

This is a primary Xaor target.

---

# Level 3 — Nation-State / Advanced Persistent Threat

Resources:

* custom hardware
* advanced exploit chains
* elite engineering teams

Capabilities:

* side-channel exploitation
* custom silicon
* supply chain compromise

Risk:
Extreme.

Xaor may increase attacker cost but cannot guarantee protection.

---

# Primary Threats

---

## Brute Force

Attacker attempts massive password guesses.

Goal:

```text
Guess → Verify → Repeat
```

Defense:

* memory hardness
* compound rounds
* adaptive cost

---

## Dictionary Attacks

Attacker uses common passwords.

Examples:

* password123
* qwerty
* admin

Defense:

Algorithm alone cannot fully solve weak secrets.

User entropy still matters.

---

## GPU Parallel Cracking

Large-scale cracking via GPUs.

Defense goal:

Reduce GPU efficiency through:

* irregular memory access
* topology mutation
* branch unpredictability

---

## ASIC Specialization

Attackers may design custom chips.

Static algorithms help ASICs.

Xaor attempts to resist by making universal optimization difficult.

---

## Rainbow Tables

Precomputed lookup tables.

Defense:

Unique salts.

Every secret must use independent salt.

---

## Collision Attacks

Goal:

Find two inputs producing same output.

Danger:

False verification.

Defense:

Strong diffusion and collision resistance.

---

## Timing Attacks

Execution timing leaks information.

Attackers measure:

* branch timing
* cache timing
* operation timing

Defense:

* constant-time comparisons
* reduced secret-dependent branching

---

# Threats Xaor Does NOT Solve

Important limitations.

---

## Malware

If attacker controls machine:

Security collapses.

Example:

Keylogger records plaintext.

Xaor cannot prevent this.

---

## Phishing

If user willingly reveals secret:

Algorithm cannot help.

---

## Social Engineering

Human manipulation bypasses cryptography.

---

## Physical Coercion

Security cannot resist all real-world force.

---

# Security Goals

Xaor aims to maximize attacker difficulty for:

* offline cracking
* verifier attacks
* hardware optimization
* large-scale guessing

It does not claim universal protection.

---

# Security Assumptions

Xaor assumes:

* secure RNG exists
* implementation is correct
* attacker lacks plaintext secret
* cryptographic primitives remain unbroken

Violation of assumptions weakens security.

---

# Summary

Threat modeling defines realistic expectations.

Xaor is primarily designed against:

* brute force
* GPU cracking
* ASIC optimization
* large-scale offline attacks
