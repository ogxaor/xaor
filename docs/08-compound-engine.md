# Compound Engine

---

# Introduction

The Compound Engine is the primary computational hardening mechanism of Xaor.

Its purpose is to repeatedly transform internal state in a recursive manner such that computational difficulty compounds over successive rounds.

This engine formalizes one of the central Xaor ideas:

> Security hardness can grow nonlinearly when outputs recursively influence future transformations.

Unlike conventional round-based systems where each round applies fixed transformations, the Compound Engine creates chained state dependencies.

---

# Motivation

Traditional systems often use fixed rounds.

Example:

```text id="kg2c4r"
Round1
Round2
Round3
Round4
```

Each round transforms state.

However, these rounds may still be structurally predictable.

Xaor introduces compounding:

```text id="6yz2ru"
Output1 → influences Output2
Output2 → influences Output3
Output3 → influences Output4
```

This creates recursive state dependence.

---

# Core Idea

Basic transformation:

[
S_{n+1}=P(S_n)
]

Where:

* (S_n) = current state
* (P) = processing function

Compound transformation adds recursive influence:

[
S_{n+1}=P(S_n \oplus f(S_{n-1},E_n))
]

Where:

* (f) = mixing function
* (E_n) = round entropy

This means current state depends on:

* current state
* previous state
* round entropy
* transformation graph

---

# Compounding Layers

Xaor compounds hardness through four layers.

---

# Layer 1 — State Dependency

Each round depends on previous state.

Without this, rounds remain independent.

Example:

```text id="xy4s8e"
S1 → S2 → S3
```

---

# Layer 2 — Recursive Mixing

Multiple historical states are mixed.

Example:

[
S_4 = P(S_3 \oplus S_2)
]

This creates temporal coupling.

---

# Layer 3 — Topology Influence

Topology may vary across rounds.

Graph structure affects processing.

---

# Layer 4 — Memory Feedback

Memory arena modifies round behavior.

This introduces additional complexity.

---

# Round Structure

Each compound round consists of stages.

---

## Stage 1 — State Fetch

Load current state.

Inputs:

* current state
* previous states
* round seed

---

## Stage 2 — Graph Traversal

Traverse topology graph.

Node execution order determined by graph metadata.

Example:

```text id="ngwbpa"
A → D → B → C
```

---

## Stage 3 — Node Processing

Each node mutates state.

Node examples:

* arithmetic
* matrix
* rotation
* memory
* symbol

---

## Stage 4 — Round Compression

Compress state into canonical form.

---

# Round Count

Number of compound rounds:

[
R
]

Recommended:

```text id="z9k0te"
8–64 rounds
```

Tradeoff:

More rounds →

* stronger resistance
* slower execution

---

# Recursive Chain Example

Example chain:

Initial seed:

[
S_0
]

Round 1:

[
S_1=P(S_0)
]

Round 2:

[
S_2=P(S_1 \oplus S_0)
]

Round 3:

[
S_3=P(S_2 \oplus S_1)
]

Round 4:

[
S_4=P(S_3 \oplus S_2)
]

Each round becomes harder to isolate analytically.

---

# Computational Cost

Compound cost roughly:

[
O(R \times G)
]

Where:

* R = rounds
* G = graph traversal complexity

Additional memory access may increase cost.

---

# Security Benefits

Compound recursion increases difficulty for:

* shortcut attacks
* optimization
* state prediction

Attackers must simulate chained dependencies.

---

# Failure Modes

---

## Too Few Rounds

Weak hardening.

---

## Excessive Rounds

Poor performance.

---

## Weak Mixing

State dependencies become predictable.

---

# Output

Compound Engine outputs transformed state:

[
S_{compound}
]

This is passed to Cycle Recycler.
