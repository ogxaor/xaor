# Benchmarks

---

# Purpose

Benchmarks measure the practical performance characteristics of Xcrypt.

They answer critical engineering questions:

* How fast is Xcrypt?
* How much memory does it consume?
* How expensive is verification?
* How well does it resist GPU parallelization?
* How does it compare to bcrypt, scrypt, and Argon2?

Benchmarking is essential because security alone is insufficient if performance becomes unusable.

---

# Benchmark Dimensions

Xcrypt must be measured across multiple axes.

---

# 1. Latency

Measures single-operation runtime.

Formula:

[
Latency = end_time - start_time
]

Measured in:

* microseconds
* milliseconds
* seconds

Operations to benchmark:

* verification
* encryption
* decryption

---

# 2. Throughput

Measures operations per unit time.

Formula:

[
Throughput = Operations / Second
]

Example:

```text id="bm1"
500 verifications/sec
```

Higher throughput benefits servers.

---

# 3. Memory Consumption

Memory usage includes:

* arena allocation
* temporary state
* graph structures

Formula:

[
Peak\ Memory
]

Measure:

* average memory
* peak memory
* memory per operation

---

# 4. CPU Utilization

Measures processor pressure.

Metrics:

* user CPU
* system CPU
* instruction count
* branch misses

Important for topology-heavy systems.

---

# 5. Cache Behavior

Xcrypt intentionally stresses memory.

Metrics:

* L1 cache misses
* L2 cache misses
* L3 cache misses
* cache eviction rate

Why?

Cache inefficiency often correlates with GPU resistance.

---

# Benchmark Hardware Profiles

Benchmarks should run on multiple systems.

---

# Low-End Consumer

Example:

* 4-core CPU
* 8–16 GB RAM

Target:

Common users.

---

# Mid-Range Workstation

Example:

* 8–16 cores
* 32–64 GB RAM

Target:

Developer systems.

---

# High-End Server

Example:

* 32+ cores
* large RAM

Target:

Enterprise deployment.

---

# GPU Testing

Essential.

Test hardware:

* gaming GPUs
* datacenter GPUs

Goal:

Measure attacker acceleration factor.

---

# Benchmark Modes

---

# Mode A — Lightweight

Parameters:

```text id="bm2"
Rounds: 8
Arena: 64 MB
Nodes: 16
```

Target:

Fast verification.

Expected latency:

```text id="bm3"
50–150 ms
```

---

# Mode B — Balanced

Parameters:

```text id="bm4"
Rounds: 16
Arena: 256 MB
Nodes: 32
```

Target:

General security.

Expected latency:

```text id="bm5"
150–700 ms
```

---

# Mode C — Heavy

Parameters:

```text id="bm6"
Rounds: 32
Arena: 1 GB
Nodes: 64
```

Target:

Maximum attacker cost.

Expected latency:

```text id="bm7"
0.7–4 sec
```

---

# Benchmark Comparison Targets

Compare against:

bcrypt
scrypt
Argon2
PBKDF2

---

# Success Criteria

Xcrypt is considered promising if:

---

## Criterion 1

Verification latency acceptable for humans:

```text id="bm8"
100–1000 ms
```

---

## Criterion 2

GPU speedup significantly reduced.

Example:

bcrypt GPU speedup:

```text id="bm9"
100×+
```

Desirable Xcrypt target:

```text id="bm10"
<10×
```

---

## Criterion 3

Memory scaling works linearly.

---

# Benchmark Tooling

Recommended tools:

* criterion-rs
* perf
* valgrind
* flamegraph

---

# Important Warning

Benchmarks alone do NOT prove security.

Fast ≠ secure
Slow ≠ secure

Performance data must be paired with cryptanalysis.
