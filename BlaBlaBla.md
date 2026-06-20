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

---

## 5. Multi-Language & Platform Support (Node.js, Python, C++) via C FFI

Xaor compiles as both a Rust library and a C-compatible dynamic shared library (`.dll` on Windows, `.so` on Linux, `.dylib` on macOS). Developers on other runtimes can bind directly to the core cryptographic engine.

### 🔌 Exported C API Functions (`src/ffi.rs`)

* `xaor_hash(password: *const c_char) -> *mut c_char`
  Hashes a UTF-8 password string. Returns a pointer to a newly allocated null-terminated string containing the PHC-formatted hash. Returns `null` on error.
* `xaor_verify(password: *const c_char, stored: *const c_char) -> i32`
  Verifies a password against a stored hash string. Returns `1` if valid, `0` if invalid, and `-1` on configuration/parsing error.
* `xaor_last_error() -> *mut c_char`
  Retrieves a pointer to the last error message string if a function returned an error. Returns `null` if no error.
* `xaor_free_string(ptr: *mut c_char)`
  **Mandatory:** Frees memory allocated by Rust for a string returned by `xaor_hash` or `xaor_last_error`.

---

### 🟢 Node.js Integration (using `ffi-napi`)

Install the FFI wrapper:
```bash
npm install ffi-napi
```

Implement the Javascript wrapper:
```javascript
const ffi = require('ffi-napi');
const path = require('path');

// Resolve path to the compiled shared library (e.g. xaor.dll, libxaor.so, or libxaor.dylib)
const libPath = path.resolve(__dirname, './target/release/xaor');

const lib = ffi.Library(libPath, {
    'xaor_hash': ['pointer', ['string']],
    'xaor_verify': ['int32', ['string', 'string']],
    'xaor_free_string': ['void', ['pointer']],
    'xaor_last_error': ['pointer', []]
});

function hashPassword(password) {
    const ptr = lib.xaor_hash(password);
    if (ptr.isNull()) {
        const errPtr = lib.xaor_last_error();
        const err = errPtr.readCString();
        lib.xaor_free_string(errPtr);
        throw new Error("Hashing failed: " + err);
    }
    const hash = ptr.readCString();
    lib.xaor_free_string(ptr);
    return hash;
}

function verifyPassword(password, hash) {
    const res = lib.xaor_verify(password, hash);
    if (res === -1) {
        const errPtr = lib.xaor_last_error();
        const err = errPtr.readCString();
        lib.xaor_free_string(errPtr);
        throw new Error("Verification failed: " + err);
    }
    return res === 1;
}

// Quick Test
const hash = hashPassword("developer-pass");
console.log("Generated hash:", hash);
console.log("Is valid:", verifyPassword("developer-pass", hash)); // true
```

---

### 🐍 Python Integration (using `ctypes`)

No external package dependencies needed.

```python
import ctypes
import os
import sys

# Resolve shared library platform extension
if sys.platform == "win32":
    lib_name = "xaor.dll"
elif sys.platform == "darwin":
    lib_name = "libxaor.dylib"
else:
    lib_name = "libxaor.so"

lib_path = os.path.abspath(os.path.join(os.path.dirname(__file__), "./target/release", lib_name))
lib = ctypes.CDLL(lib_path)

# Configure parameter and return types
lib.xaor_hash.argtypes = [ctypes.c_char_p]
lib.xaor_hash.restype = ctypes.c_void_p  # Raw pointer returned to handle deallocation

lib.xaor_verify.argtypes = [ctypes.c_char_p, ctypes.c_char_p]
lib.xaor_verify.restype = ctypes.c_int32

lib.xaor_free_string.argtypes = [ctypes.c_void_p]
lib.xaor_free_string.restype = None

lib.xaor_last_error.argtypes = []
lib.xaor_last_error.restype = ctypes.c_void_p

def hash_password(password: str) -> str:
    ptr = lib.xaor_hash(password.encode('utf-8'))
    if not ptr:
        err_ptr = lib.xaor_last_error()
        err = ctypes.cast(err_ptr, ctypes.c_char_p).value.decode('utf-8')
        lib.xaor_free_string(err_ptr)
        raise Exception(f"Hashing failed: {err}")
    
    hash_str = ctypes.cast(ptr, ctypes.c_char_p).value.decode('utf-8')
    lib.xaor_free_string(ptr)
    return hash_str

def verify_password(password: str, hash_str: str) -> bool:
    res = lib.xaor_verify(password.encode('utf-8'), hash_str.encode('utf-8'))
    if res == -1:
        err_ptr = lib.xaor_last_error()
        err = ctypes.cast(err_ptr, ctypes.c_char_p).value.decode('utf-8')
        lib.xaor_free_string(err_ptr)
        raise Exception(f"Verification failed: {err}")
    return res == 1

# Quick Test
hash_val = hash_password("developer-pass")
print("Generated hash:", hash_val)
print("Is valid:", verify_password("developer-pass", hash_val)) # True
```

---

## 6. Multi-Platform Package Distribution Roadmap

To make Xaor available as native packages on different language registries (so developers can run `npm install xaor` or `pip install xaor` without compiling Rust from source), use these modern workflows:

### 📦 Node.js (NPM Registry)

#### Option A: Native Node Addons (Recommended for Performance)
Use [napi-rs](https://napi.rs/) to compile Rust directly to a Node.js binary module (`.node`).
1. **Setup:** Install the CLI: `npm install -g @napi-rs/cli`.
2. **Initialize:** Run `napi-rs init` to configure your Rust project for N-API.
3. **Github Actions Workflow:** `napi-rs` provides a pre-built GitHub Actions template that automatically cross-compiles your Rust engine on push for Windows, macOS, and Linux (both x64 and ARM architectures).
4. **Publish:** It generates platform-specific package variants (e.g., `@xaor/core-win32-x64`) and a main package (`xaor`) that dynamically loads the correct binary for the user's OS.

#### Option B: WebAssembly (Recommended for Web Browsers & Edge Runtimes)
Use [wasm-pack](https://rustwasm.github.io/wasm-pack/) to compile the Rust logic into WebAssembly (`.wasm`) + a Javascript glue layer.
1. **Build:** Run `wasm-pack build --target nodejs` (or `--target bundler` for browsers).
2. **Publish:** Run `wasm-pack publish` to publish directly to NPM.
3. *Note:* WebAssembly has a slight execution speed penalty compared to native FFI, but is 100% portable and runs in browsers, Cloudflare Workers, and serverless runtimes.

---

### 🐍 Python (PyPI Registry)

Use [Maturin](https://www.maturin.rs/) paired with [PyO3](https://pyo3.rs/) to create python extension modules.
1. **Setup:** Install Maturin: `pip install maturin`.
2. **Configure:** Add PyO3 dependencies to your `Cargo.toml` and write your Python interface in `src/lib.rs` (using `#[pymodule]` macros).
3. **Build Wheels:** Run `maturin build --release` to generate platform-specific wheel files (`.whl`).
4. **Publish:** Run `maturin publish` to upload the wheels to PyPI. Users can then run `pip install xaor` and receive the pre-compiled binary matching their OS and Python version instantly.

---

### 🛡️ C / C++ header generation

To let C/C++ developers compile directly against Xaor's FFI library:
1. Use **`cbindgen`** to parse your Rust FFI definitions and output a C header file (`xaor.h`) automatically.
2. Install: `cargo install --force cbindgen`
3. Generate:
   ```bash
   cbindgen --config cbindgen.toml --crate xaor --output xaor.h
   ```
4. Distribute `xaor.h` alongside the precompiled dynamic libs (`xaor.dll`, `libxaor.so`, `libxaor.dylib`) via GitHub Releases.


