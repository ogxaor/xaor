# Xaor Syntax & Environment Reference Guide

This guide is compiled for developers implementing or interacting with the **Xaor Cryptographic Engine** CLI and Rust library. It covers shell-specific syntax issues, command-line arguments, environment configurations, and library usage.

---

## 1. Shell Environments & The `$` Symbol

Xaor utilizes the standard **PHC (Password Hashing Competition) String Format** for its output. Because of this, hashes contain `$` delimiters:
`$xaor$v=3$m=256$r=16$n=32$l=1$o=std$<salt>$<hash>`

Many command-line shells parse the `$` symbol as a variable identifier. To verify hashes using the CLI, you must format the hash argument correctly according to your shell environment.

### 🐚 PowerShell (Windows)
PowerShell parses double-quoted strings for variables. 
* **Correct:** Use **single quotes** (`'`) to pass the string literally.
  ```powershell
  xaor verify "my-password" '$xaor$v=3$m=256$r=16$n=32$l=1$o=std$fnBPCGtZqTL...$DIdrV...'
  ```
* **Alternative:** Escape each `$` using a backtick (`` `$ ``).
  ```powershell
  xaor verify "my-password" "`$xaor`$v=3`$m=256..."
  ```

### 🐧 Bash / Zsh (Linux & macOS)
Bash/Zsh also expands variables within double quotes.
* **Correct:** Use **single quotes** (`'`) to prevent shell expansion.
  ```bash
  xaor verify "my-password" '$xaor$v=3$m=256$r=16$n=32$l=1$o=std$fnBPCGtZqTL...$DIdrV...'
  ```
* **Alternative:** Escape each `$` using a backslash (`\$`).
  ```bash
  xaor verify "my-password" "\$xaor\$v=3\$m=256..."
  ```

### 💻 Command Prompt (Windows cmd.exe)
`cmd.exe` does not use `$` for variables (it uses `%var%` instead).
* **Correct:** Standard double quotes (`"`) work perfectly.
  ```cmd
  xaor verify "my-password" "$xaor$v=3$m=256$r=16$n=32$l=1$o=std$fnBPCGtZqTL...$DIdrV..."
  ```

---

## 2. CLI Command & Parameters Reference

The Xaor CLI provides multiple utility commands:

| Command | Arguments | Description | Example |
| :--- | :--- | :--- | :--- |
| **`hash`** | `<password>` | Hashes a password and prints the PHC string | `xaor hash "my-password"` |
| **`verify`** | `<password> <stored-hash>` | Verifies a password against a stored hash string | `xaor verify "pass" '$xaor$...'` |
| **`bench`** | `[iterations]` | Benchmarks hash speed (default: 10 runs) | `xaor bench 5` |
| **`config`** | *None* | Shows the active configuration parameters | `xaor config` |
| **`errors`** | *None* | Catalog of all system error codes | `xaor errors` |
| **`mode`** | `<hash\|encrypt>` | Confirms validation of a mode parameter | `xaor mode hash` |

### 🛠️ Profile Presets (`--profile`)
Specify a preset profile via the `--profile` flag to adjust performance characteristics:
```bash
xaor --profile interactive hash "my-password"
```

| Profile Name | Mixing Rounds | Memory Limit | Target Latency | Typical Use Case |
| :--- | :---: | :---: | :---: | :--- |
| **`interactive`** | `12` | `64 MB` | ~100-200 ms | Fast login verification / Web UIs |
| **`standard`** *(default)* | `16` | `256 MB` | ~300-500 ms | General password hashing |
| **`high-security`** | `24` | `512 MB` | ~1-2 sec | High-security backend archives |
| **`server`** | `16` | `256 MB` | ~300-500 ms | Multithreaded authentication servers (4 lanes) |

### 🌍 Override Environment Variables
You can dynamically override Xaor's configuration parameters via environment variables without passing arguments to the binary:

* `XAOR_PROFILE`: Set preset profile (`interactive`, `standard`, `high-security`, `server`).
* `XAOR_ROUNDS`: Number of chaotic mixing rounds.
* `XAOR_MEMORY`: Size of memory arena (in MB).
* `XAOR_NODES`: Number of topology nodes in the evaluation graph.
* `XAOR_LANES`: Execution lanes for parallel processing.
* `XAOR_MODE`: Execution mode (`hash` or `encrypt`).
* `XAOR_PEPPER`: Server-side secret pepper to include in the hash (not stored).

---

## 3. Rust Developer API Integration

Developers using `xaor` as a cargo dependency can use the API in two ways:

### A. One-Liners (Associated Functions)
For simple applications, use the `Xaor` one-liners. This uses standard safe defaults automatically.

```rust
use xaor::Xaor;

fn main() -> Result<(), xaor::XaorError> {
    let password = "developer-secret-passphrase";

    // 1. Hash a password
    let stored_hash = Xaor::quick_hash(password)?;
    println!("Stored Hash: {}", stored_hash);

    // 2. Verify a password (parameters are parsed out of the hash itself)
    let is_valid = Xaor::quick_verify(password, &stored_hash)?;
    assert!(is_valid);
    
    Ok(())
}
```

### B. Custom Builder Pattern (Object Methods)
For customizable parameters and instances, construct an `Xaor` instance via `XaorBuilder`.

```rust
use xaor::Xaor;

fn main() -> Result<(), xaor::XaorError> {
    // Construct a custom configuration
    let hasher = Xaor::builder()
        .memory_mb(128)        // Custom memory limit
        .rounds(18)            // Custom mixing rounds
        .pepper_str("secret")  // Optional system pepper
        .quantum()             // Output 128-byte quantum-resistant hashes
        .build()?;

    let password = "secure-pass-phrase";
    
    // Hash using the object instance method
    let hash = hasher.hash_password(password)?;
    
    // Verify using the object instance method
    let is_valid = hasher.verify_password(password, &hash)?;
    assert!(is_valid);

    Ok(())
}
```

---

## 4. Stored Hash Format Anatomy

A Xaor hash string contains all the parameters necessary to replicate the hashing sequence during verification:

```
 $xaor$v=3$m=256$r=16$n=32$l=1$o=std$fnBPCGtZq...$DIdrV5qNZ...
 ──┬── ─┬─ ──┬── ─┬─ ─┬─ ─┬─ ─┬── ──┬── ───┬───
   │    │    │    │   │   │   │     │      └─ Base64 SHA3 Finalized Hash
   │    │    │    │   │   │   │     └─────── Base64 Random Salt
   │    │    │    │   │   │   └───────────── Output Mode (std / qtm)
   │    │    │    │   │   └───────────────── Parallel Lanes
   │    │    │    │   └───────────────────── Graph Nodes
   │    │    │    └───────────────────────── Chaotic Rounds
   │    │    └────────────────────────────── Memory Arena (MB)
   │    └─────────────────────────────────── Xaor Protocol Version
   └──────────────────────────────────────── Algorithm Identifier (XAOR)
```
