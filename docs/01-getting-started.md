# Quick Start

---

## 30-Second Installation

Xaor is designed to be highly portable and runs as a native package across multiple programming environments:

| Language | Registry | Installation Command |
| :--- | :--- | :--- |
| **Rust** | Crates.io | `cargo add xaor` |
| **Node.js** | NPM | `npm install xaorjs` |
| **Python** | PyPI | `pip install xaor` |
| **Dart** | Pub.dev | `dart pub add xaor` |
| **Go** | Go Modules | `go get github.com/ogxaor/xaor/go` |

---

## Hello World Code Example

Once installed, hashing and verification are extremely straightforward. Here is the standard API flow:

### Rust
```rust
use xaor::Xaor;

fn main() -> Result<(), xaor::XaorError> {
    // 1. Hash a password with standard safe parameters
    let hash = Xaor::quick_hash("my-secure-password")?;
    println!("Hash: {}", hash);

    // 2. Verify it against the password
    let is_valid = Xaor::quick_verify("my-secure-password", &hash)?;
    assert!(is_valid);
    
    Ok(())
}
```

---

## Command Line Quick Start

If you prefer to run Xaor from the terminal, you can perform hashes, benchmarks, and verification directly:

### 1. Hash a secret
```bash
xaor hash "my-password"
```

### 2. Verify a stored hash
```bash
# Note: Use single quotes for shell safety
xaor verify "my-password" '$xaor$v=3$m=256$r=16$n=32$l=1$o=std$saltbytes...$hashbytes...'
```

### 3. Benchmark performance
```bash
# Benchmarks hashing performance (e.g. 5 iterations)
xaor bench 5
```
