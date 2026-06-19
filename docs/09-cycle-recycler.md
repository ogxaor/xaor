# Cycle Recycler

---

# Introduction

The Cycle Recycler is one of the most distinctive components of Xcrypt.

Its purpose is to recycle intermediate computational states back into future computation stages.

This allows internal computation to evolve recursively.

Traditional cryptographic systems rarely allow round outputs to mutate future structure.

Xcrypt intentionally enables this behavior.

---

# Motivation

Fixed computation structure creates predictability.

Example:

```text id="q9bx1n"
Round 1
Round 2
Round 3
```

Structure remains unchanged.

Cycle recycling changes this:

```text id="fcf6y8"
Round Output
     ↓
Mutate Future Round
```

This creates adaptive structural evolution.

---

# Definition

Recycler function:

[
R(S_n)=M_n
]

Where:

* (S_n) = round output
* (M_n) = mutation directives

Mutation directives determine how future computation changes.

---

# Recyclable State Components

Not all state is recycled.

Candidate recyclable components:

* hash fragments
* matrix coefficients
* traversal offsets
* rotation schedules
* memory indices

Sensitive raw state may remain protected.

---

# Mutation Targets

Recycler may modify several subsystems.

---

# Target 1 — Topology Mutation

Graph structure changes.

Examples:

* edge reorder
* traversal reorder
* node replacement

Example:

Before:

```text id="gvx4i0"
A → B → C
```

After:

```text id="rjgcqf"
A → C → B
```

---

# Target 2 — Matrix Mutation

Recycler changes matrix coefficients.

Example:

[
M_1 \rightarrow M_2
]

This alters diffusion behavior.

---

# Target 3 — Rotation Mutation

Rotation schedules change.

Example:

Before:

```text id="xhg4s4"
ROL 7
ROR 13
```

After:

```text id="8eq7i1"
ROL 19
ROR 11
```

---

# Target 4 — Memory Window Mutation

Active memory segments change.

This affects access patterns.

---

# Recycling Pipeline

---

## Step 1 — Capture State Snapshot

Intermediate round output selected.

---

## Step 2 — Extract Mutation Signals

Reducer extracts mutation values.

Example:

```text id="s9qewk"
bits 12–28
bits 40–55
```

---

## Step 3 — Generate Mutation Directives

Signals converted into actionable mutations.

---

## Step 4 — Apply Constraints

Mutation must obey safety rules.

---

# Mutation Constraints

Critical for determinism.

---

## Constraint 1 — Deterministic Mutation

Same inputs must produce same mutation.

Required for verification and decryption.

---

## Constraint 2 — Bounded Mutation

Mutation cannot destabilize computation.

---

## Constraint 3 — Valid Topology

Graph must remain executable.

---

# Security Benefits

Cycle recycling improves resistance against:

* static optimization
* repeated attack profiling
* hardware specialization

Attackers must model evolving computation.

---

# Failure Modes

---

## Excessive Mutation

Can cause instability.

---

## Weak Mutation

Little security gain.

---

## Broken Determinism

Catastrophic for decryption and verification.

---

# Output

Recycler outputs mutation directives:

[
M_n
]

These mutate future computation cycles.
