# Compilation & Platforms

---

## Target Architectures & Library Outputs

Xaor compiles as both a native Rust library and a C-compatible shared library. To support multiple developer stacks, the build system outputs the following files depending on the platform:

| Platform | Library Type | Output File | Linker Behavior |
| :--- | :--- | :--- | :--- |
| **Windows** | Dynamic Library | `xaor.dll` | Standard Win32 DLL |
| **Linux** | Shared Object | `libxaor.so` | ELF Shared Library |
| **macOS** | Dynamic Library | `libxaor.dylib` | Mach-O Dynamic Library |
| **Web / Node.js** | WebAssembly | `xaor.wasm` | WebAssembly byte code |

---

## Compilation Guides

Ensure you have the Rust compiler `rustc` and package manager `cargo` installed.

### Native Local Build
Compile the native library for your active platform using:
```bash
cargo build --release
```
This produces both the static library (`.rlib`) and the dynamic shared library (`.dll`/`.so`/`.dylib`) in the `target/release/` directory.

### Optimizing Compiler Flags
For maximum performance (e.g., speeding up graph transformations and RAM passes), compile with CPU-specific instructions enabled and Link-Time Optimization (LTO):
```bash
RUSTFLAGS="-C target-cpu=native" cargo build --release
```
Add the following to your `Cargo.toml` for smaller binaries and faster runtime:
```toml
[profile.release]
lto = true
opt-level = 3
codegen-units = 1
panic = "abort"
```

### WebAssembly Compilation
Compile the Rust codebase into portable WebAssembly for edge workers and browsers:
```bash
# Install wasm-pack if you haven't already
cargo install wasm-pack

# Build WASM package for browsers
wasm-pack build --target web

# Build WASM package for Node.js
wasm-pack build --target nodejs
```

---

## FFI Symbols & C Type Map

When loading the shared library in non-Rust platforms (e.g., Python `ctypes`, Node `ffi-napi`, Dart `dart:ffi`), map the following `extern "C"` functions:

### 1. Error Handling
* `xaor_last_error() -> *mut c_char`
  Returns a pointer to a null-terminated UTF-8 string containing the last error message in the active thread context. Returns `null` if no error.
* `xaor_free_string(ptr: *mut c_char) -> void`
  Deallocates a string pointer previously returned by the library. **Must be called on all returned string pointers to avoid leaks.**

### 2. Core Hashing
* `xaor_hash(password: *const c_char) -> *mut c_char`
  Hashes a plaintext string. Uses parameters defined in environment variables. Returns a C-string representing the stored hash.
* `xaor_verify(password: *const c_char, stored: *const c_char) -> i32`
  Verifies a password against a stored PHC hash. Returns `1` (valid), `0` (invalid), or `-1` (error).

---

## WebAssembly JavaScript Exports

The compiled WASM wrapper (`xaor.wasm` + JS glue layer) exports the following JavaScript signatures:

```typescript
export function hash_password(password: string): string;
export function verify_password(password: string, stored: string): boolean;
export function xnonce_generate(length: number): string;
export function xtoken_generate(length: number, prefix?: string): string;
export function xid_generate(length: number, prefix?: string, checksum?: boolean): string;
export function xid_validate(id: string, prefix?: string, checksum?: boolean): boolean;
```
These functions run in sandboxed JavaScript threads without loading external FFI library files.
