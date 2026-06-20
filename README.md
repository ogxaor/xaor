<div align="center">
<pre class=" bg-black">
 ██╗  ██╗  █████╗   ██████╗  ██████╗ 
 ╚██╗██╔╝ ██╔══██╗ ██╔═══██╗ ██╔══██╗
  ╚███╔╝  ███████║ ██║   ██║ ██████╔╝
  ██╔██╗  ██╔══██║ ██║   ██║ ██╔══██╗
 ██╔╝ ██╗ ██║  ██║ ╚██████╔╝ ██║  ██║
 ╚═╝  ╚═╝ ╚═╝  ╚═╝  ╚═════╝  ╚═╝  ╚═╝
</pre>
<p><strong>memory-hard cryptographic engine</strong> &nbsp;•&nbsp; <code>v0.2.1</code></p>
</div>


[![Crates.io](https://img.shields.io/badge/crates.io-xaor-orange)](https://crates.io/crates/xaor)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Status: Alpha](https://img.shields.io/badge/Status-Alpha-yellow)]()

---

> [!NOTE]
> **Open Source Project**: Xaor is free and open-source software licensed under the MIT License. Anyone is free to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the software without restriction.

---


## 30-Second Start

```rust
// Hash a password — zero config needed:
let hash = Xaor::quick_hash("my-password")?;

// Verify it:
let ok = Xaor::quick_verify("my-password", &hash)?;

// That's it. You're done.
```

---

## Multi-Language Installation

Xaor is available as a native package across multiple language registries:

| Language | Registry | Installation Command |
| :--- | :--- | :--- |
| **Rust** | Crates.io | `cargo add xaor` |
| **Node.js** | NPM | `npm install xaorjs` |
| **Python** | PyPI | `pip install xaor` |
| **Dart** | Pub.dev | `dart pub add xaor` |
| **Flutter** | Pub.dev | `flutter pub add xaor` |
| **Go** | Go Modules | `go get github.com/ogxaor/xaor/go` |

---

## Why Xaor Exists

Every major password hashing algorithm has a fixed, predictable structure.

- **bcrypt** has no memory hardness — a modern GPU can try 1M hashes/sec.
- **Argon2id** is excellent but its memory access pattern is sequential and well-studied.
- **PBKDF2/SHA** are GPU-friendly. Do not use for passwords.

Xaor is different: **its internal computation graph changes with every password+salt pair.**
An attacker who has studied how to optimise against Xaor's structure finds a different
structure every time. There is no fixed S-box, no fixed topology, no fixed mixing schedule.

Additionally, Xaor stresses **both RAM and CPU simultaneously** — not one or the other.
This means GPU farms (low cache, shared memory bandwidth) are particularly expensive to run.

---

## Architecture

```
Input ──► SeedStage ──► TopologyStage ──► CompoundStage ──► ChaosStage
                              │                  │                │
                         BLAKE3-keyed       Per-node         Integer
                         graph (2^256      subkey ops       xorshift
                         topologies)       (no constants)   (no f64)
                              │
                         MemoryStage ──► RecyclerStage ──► FinalizerStage
                              │
                     3-pass: forward      BLAKE3-XOF       64 or 128
                     + backward +         keyed cycles     byte output
                     keyed arena
```

### Stage Breakdown

| Stage | What it does | Why it matters |
|---|---|---|
| **SeedStage** | BLAKE3-XOF(domain \|\| input \|\| salt \|\| pepper?) → 64 bytes | Domain-separated 512-bit seed with optional server-side pepper |
| **TopologyStage** | BLAKE3-keyed computation graph (32 nodes × 32-byte subkey) | 2^256 possible topologies per password — no fixed structure to optimise against |
| **CompoundStage** | Keyed 64-bit word transforms per graph node per round | Secret-dependent — attacker cannot reproduce without knowing the password |
| **ChaosStage** | Integer xorshift64 + Knuth multiplicative mixing | CPU pipeline stress; no f64 timing side-channel |
| **RecyclerStage** | BLAKE3-XOF keyed cycles + 64-bit word diffusion | Full-length keystream per cycle; guaranteed nonzero rotations |
| **MemoryStage** | 3-pass arena (forward + backward + keyed BLAKE3) | Dual-chain dependency forces full arena in RAM; TMTO is mathematically impossible |
| **FinalizerStage** | 64-byte Standard or 128-byte Quantum dual-XOF output | Quantum mode: two independent BLAKE3-XOF streams for 512-bit post-quantum security |

> **Parallel Branching (Lanes)**: The entire pipeline above can be split into multiple independent parallel lanes using `rayon`. This allows legitimate servers to hash passwords extremely quickly across multi-core CPUs, while forcing attackers to dedicate entire cores per guess.

---

## Usage

### One-Liner API

```rust
use xaor::Xaor;

// Hash
let hash = Xaor::quick_hash("hunter2")?;

// Verify
let ok = Xaor::quick_verify("hunter2", &hash)?;
```

### Builder API

```rust
use xaor::{Xaor, XaorConfig};

let xaor = Xaor::builder()
    .memory_mb(512)       // RAM cost per hash
    .rounds(24)           // CPU rounds
    .nodes(48)            // topology graph nodes
    .lanes(4)             // parallel execution branches
    .pepper_str("$MY_SERVER_SECRET")  // server-side secret (not in DB)
    .build()?;

let hash = xaor.hash_password("hunter2")?;
let ok   = xaor.verify_password("hunter2", &hash)?;
```

### Quantum Mode (128-byte output)

```rust
let xaor = Xaor::builder()
    .quantum()           // Enable 128-byte dual-XOF output
    .build()?;

let hash = xaor.hash_password("hunter2")?;
// Hash string contains o=qnt to signal quantum mode
```

### Auto-Tune to Your Hardware

```rust
// Picks memory/rounds to complete in ~300ms on the current machine
let config = XaorConfig::auto_tune();
println!("{}", config.preview());
let xaor = Xaor::new(config)?;
```

### Preset Profiles

```rust
use xaor::XaorConfig;

XaorConfig::interactive()   // ~100-200ms — login forms
XaorConfig::standard()      // ~300-500ms — server APIs (DEFAULT)
XaorConfig::server()        // ~500ms, high concurrency
XaorConfig::high_security() // ~1-2s — admin accounts, key derivation
```

### Environment Variables

```bash
XAOR_PROFILE=standard        # interactive | standard | server | high_security
XAOR_ROUNDS=16               # integer
XAOR_MEMORY=256              # MB
XAOR_NODES=32                # graph nodes
XAOR_LANES=4                 # parallel execution branches
XAOR_OUTPUT_MODE=quantum     # standard (default) | quantum
XAOR_PEPPER=my-server-secret # optional server-side secret
```

### Pepper — Server-Side Secret

If your database is leaked, memory-hard hashes can still be cracked offline.
A **pepper** is a server-side secret (stored in env/HSM, NOT in the database)
that makes offline cracking impossible even with full hash+salt access.

```rust
let xaor = Xaor::builder()
    .pepper(std::env::var("PEPPER_KEY").unwrap().into_bytes())
    .build()?;
```

The pepper is mixed into the seed with a domain separator (`"xaor.seed.pepper.v1"`)
and is never stored in the hash string.

---

## Experimental Subsystems

Xaor ships a complete cryptographic toolkit — not just a hasher.

| Module | What it does | Example |
|---|---|---|
| **xnonce** | Cryptographically secure nonce generation | `NonceEngine::new(32)?.generate_hex()?` |
| **xtoken** | Secure random token with optional prefix | `TokenEngine::with_prefix(32, Some("tk"))?.generate()?` |
| **xid** | Context-aware deterministic ID with checksum | `XidEngine::new(config)?.generate()?` |
| **xcipher** | Authenticated AEAD encryption (BLAKE3-based) | `CipherEngine::new().encrypt(data, key, nonce, ad)` |
| **xvault** | Local secret storage with master-key keystream | `VaultEngine::new(path).store(name, secret, key)` |
| **xproof** | Proof-of-work challenge/solve/verify | `ProofEngine::new(cfg)?.challenge(subject)?` |

---

## Hash String Format

```
$xaor$v=3$m=256$r=16$n=32$l=4$o=std$<salt_base64>$<hash_base64>
  │    │    │     │    │    │ │    │              └─ 64 or 128 bytes
  │    │    │     │    │    │ │    └─ 32-byte random salt
  │    │    │     │    │    │ └─ output mode: std or qnt
  │    │    │     │    │    └─ parallel lanes
  │    │    │     │    └─ node count
  │    │    │     └─ rounds
  │    │    └─ memory in MB
  │    └─ version
  └─ algorithm identifier
```

The hash string is fully self-describing — no external config needed to verify.

---

## Security Properties

| Property | Value |
|---|---|
| Output length | 64 bytes (Standard) or 128 bytes (Quantum) |
| Salt length | 32 bytes (256-bit) |
| Memory hardness | ✅ Yes — 3-pass arena with BLAKE3-keyed init |
| Parallel Branching | ✅ Yes — independent parallel lanes |
| TMTO resistance | ✅ Forward + Backward dual hash chains |
| GPU resistance | ✅ Random addressing + Integer chaos (no SIMD-friendly patterns) |
| ASIC resistance | ✅ BLAKE3 keyed mode + xorshift chaos requires full CPU |
| Side-channel timing | ✅ Integer-only — no f64 denormal risk |
| Memory zeroing | ✅ `zeroize` on Seed, EntropyVector, MemoryArena |
| Constant-time verify | ✅ Bitwise XOR accumulator, no early exit |
| Pepper support | ✅ Domain-separated server-side secret |
| Post-quantum | ✅ 512-bit standard; 1024-bit Quantum mode |

---

## CLI

```bash
xaor hash "mypassword"
xaor verify "mypassword" "$xaor$v=2$..."
xaor bench 5
xaor --profile interactive config
xaor errors
```

---

## Comparison vs. Industry Standards

| | Xaor | bcrypt | Argon2id | SHA-3 |
|---|---|---|---|---|
| Memory hardness | ✅ 3-pass | ❌ None | ✅ 1-pass | ❌ None |
| Parallel lanes | ✅ Yes | ❌ No | ✅ Yes | ❌ No |
| Dual-chain TMTO resistance | ✅ Yes | ❌ N/A | ⚠️ Partial | ❌ N/A |
| Dynamic topology | ✅ Per-hash | ❌ Fixed | ❌ Fixed | ❌ Fixed |
| Keyed compound ops | ✅ Yes | ❌ Fixed | ❌ Fixed | ❌ Fixed |
| Integer chaos (no f64) | ✅ Yes | ❌ N/A | ❌ N/A | ❌ N/A |
| Arena memory zeroed | ✅ Yes | ❌ N/A | ⚠️ Impl-dep | ❌ N/A |
| Pepper support | ✅ Built-in | ❌ Manual | ❌ Manual | ❌ N/A |
| Quantum mode (128B) | ✅ Opt-in | ❌ No | ❌ No | ✅ Yes |
| Public audit | ❌ Alpha | ✅ 30yr | ✅ PHC | ✅ NIST |

---

## Security Warning

Xaor is experimental (v0.2.1). It has not undergone academic peer review or
formal cryptanalysis. **Do not use Xaor as the sole protection layer in production
systems until a public audit is completed.**

It is, however, an excellent candidate for:
- Research and cryptographic exploration
- Non-critical internal tooling
- Supplementary hashing layer (alongside Argon2id)
- Learning about memory-hard function design

---

## License & Open Source

This project is fully open-source and released under the **MIT License**.

Under this license, anyone is free to:
- **Use** the software for private, commercial, or educational purposes.
- **Distribute** the software and copy/sub-license it to others.
- **Modify** the codebase and write derived applications without restriction.
- **Integrate** Xaor into your own projects (commercial or open-source).

We welcome forks, contributions, and feedback from the community! See the [LICENSE](LICENSE) file for the full legal text.

