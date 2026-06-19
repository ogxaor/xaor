# Memory Arena

---

# Introduction

The Memory Arena provides Xaor with memory-hard resistance.

Its goal is to increase the cost of hardware acceleration by forcing expensive memory operations.

Modern attackers often possess enormous computational throughput.

Arithmetic alone is cheap.

Memory bandwidth is expensive.

Xaor exploits this asymmetry.

---

# Why Memory Hardness Matters

GPUs excel at:

* arithmetic throughput
* SIMD workloads
* parallel execution

However, irregular memory access harms GPU efficiency.

Memory-hard systems exploit this weakness.

---

# Arena Definition

Memory Arena is a large allocated memory region.

Represented as:

[
A=[a_1,a_2,a_3,\dots,a_n]
]

Each element stores state fragments.

---

# Arena Size

Configurable.

Recommended prototype sizes:

Small:

```text id="4sxfz3"
64 MB
```

Medium:

```text id="ahg70n"
256 MB
```

Heavy:

```text id="yzqtxm"
1–4 GB
```

Larger arena →

higher attack cost.

---

# Arena Initialization

Arena initialized using seed-derived data.

Pipeline:

```text id="k6s8mu"
seed
 ↓
expand bytes
 ↓
fill arena
```

Each cell receives pseudorandom content.

---

# Windowing Concept

Xaor introduces memory windows.

Instead of accessing full arena uniformly, computation focuses on rotating windows.

Example arena:

```text id="w2rvru"
W1 W2 W3 W4 W5 W6
```

Each window is a segment.

---

# Active Windows

Only selected windows are active per round.

Example:

Round 1:

```text id="bdhvcg"
W1 W3
```

Round 2:

```text id="1n2mp4"
W4 W6
```

Round 3:

```text id="ov6qsw"
W2 W5
```

This increases irregularity.

---

# Memory Operations

Xaor uses several operations.

---

## Read

Load arena data.

---

## Write

Modify arena cell.

---

## Swap

Exchange cells.

---

## Scramble

Apply local transformations.

---

## Fold

Combine multiple cells.

---

# Address Selection

Addresses derived from:

* round seed
* state fragments
* recycler mutations

Formula:

[
addr=f(seed,state,round)
]

Goal:

Make address prediction difficult.

---

# Memory Pressure

Memory cost approximated as:

[
Cost = Memory \times AccessFrequency
]

Higher memory pressure increases cracking cost.

---

# GPU Resistance

Memory arena attempts to reduce:

* SIMD efficiency
* cache locality
* memory coalescing

Irregular access hurts large-scale parallel cracking.

---

# Security Benefits

Memory arena improves resistance against:

* GPU crackers
* ASIC optimization
* parallel brute-force attacks

---

# Failure Modes

---

## Arena Too Small

Low memory hardness.

---

## Predictable Access Pattern

Attackers optimize caching.

---

## Too Large for Legitimate Use

User performance suffers.

---

# Output

Memory Arena continuously mutates shared computational state during compound rounds.

It serves as a dynamic resistance layer rather than a standalone engine.
