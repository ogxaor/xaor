# Mathematical Foundation

---

# Introduction

Xaor relies on several mathematical domains:

* information theory
* probability
* graph theory
* matrix algebra
* modular arithmetic
* computational complexity

These form the mathematical foundation of the engine.

---

# Entropy

Entropy measures uncertainty.

For discrete outcomes:

[
H(X) = - \sum p(x) \log_2 p(x)
]

Where:

* (X) = random variable
* (p(x)) = probability of outcome

Higher entropy means greater unpredictability.

---

## Example: Fair Coin

Outcomes:

* Heads
* Tails

Probability:

[
0.5
]

Entropy:

[
H = 1 \text{ bit}
]

---

# Search Space

Brute-force difficulty depends on search space.

Formula:

[
N^L
]

Where:

* N = symbol set size
* L = length

Example:

4-digit PIN:

[
10^4 = 10,000
]

ASCII password (95 chars, length 8):

[
95^8
]

Huge increase.

---

# Computational Complexity

Algorithm cost is measured using asymptotic notation.

Examples:

[
O(n)
]

Linear.

[
O(n^2)
]

Quadratic.

[
O(n \log n)
]

Sub-quadratic.

Xaor intentionally introduces expensive operations to increase cracking cost.

---

# State Representation

Xaor operates on internal state vectors.

Example:

[
S = [s_1, s_2, s_3, ..., s_n]
]

Each transformation mutates state.

---

# Transformation Functions

General transformation:

[
S' = T(S)
]

Where:

* S = current state
* T = transformation function
* S' = next state

Multiple transformations:

[
S_{n+1}=T_n(S_n)
]

---

# Compound Transformations

Core Xaor concept.

Recursive definition:

[
S_{n+1}=P(S_n \oplus f(S_{n-1},E_n))
]

Where:

* (P) = processor
* (E_n) = entropy vector
* (f) = mixing function

This formula formalizes compound hardness.

---

# Matrix Mixing

Matrices provide diffusion.

Example matrix:

[
M =
\begin{bmatrix}
a & b \
c & d
\end{bmatrix}
]

Transform:

[
S' = M \times S
]

Benefits:

* mixing
* diffusion
* state coupling

---

# Graph Theory

Xaor uses computational graphs.

Graph:

[
G=(V,E)
]

Where:

* V = nodes
* E = edges

Nodes represent transformation units.

Edges represent traversal order.

Topology mutation changes graph structure.

---

# Memory Hardness

Memory hardness measures RAM cost.

Simplified cost:

[
C = Time \times Memory
]

Goal:

Increase attacker resource consumption.

Example:

1 second × 1 GB

is more expensive than:

1 second × 1 MB

---

# Avalanche Effect

Good cryptographic diffusion requires small input changes to cause large output changes.

Ideal behavior:

1 bit changed in input →

~50% output bits changed

This is called avalanche property.

---

# Collision Resistance

Collision:

[
x \neq y \quad but \quad H(x)=H(y)
]

Strong systems make collisions infeasible.

---

# Summary

Mathematical pillars of Xaor:

* entropy theory
* probability
* state transformations
* recursion
* graph topology
* matrix diffusion
* memory hardness

These foundations enable the engine architecture defined in later chapters.
