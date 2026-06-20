# Xaor Python Package

Python bindings for the **Xaor Cryptographic Engine** — an adaptive, memory-hard, chaos-keyed password hashing algorithm.

## Installation

```bash
pip install xaor
```

## Quick Start

```python
from xaor import hash_password, verify_password

# 1. Hash a password
hashed = hash_password("my-secure-password")
print("Hash:", hashed)

# 2. Verify a password
is_valid = verify_password("my-secure-password", hashed)
print("Is valid:", is_valid)  # True
```
