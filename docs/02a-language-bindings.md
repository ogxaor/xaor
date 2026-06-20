# Language Bindings

---

## Rust Integration

### Installation & Setup
To integrate Xaor in a Rust project, add it to your dependencies in `Cargo.toml`:
```toml
[dependencies]
xaor = "0.2.1"
```
Or run the installation command in your cargo workspace:
```bash
cargo add xaor
```

### Custom Hashing Parameters
```rust
use xaor::{Xaor, XaorConfig};

fn main() -> Result<(), xaor::XaorError> {
    let hasher = Xaor::builder()
        .memory_mb(128)        // Custom memory limit in MB
        .rounds(18)            // Custom chaotic mixing rounds
        .nodes(40)             // Custom graph node count
        .lanes(4)              // Parallel threading lanes
        .pepper_str("secret")  // Server-side pepper
        .quantum()             // Output 1024-bit post-quantum resistant hashes
        .build()?;

    let hash = hasher.hash_password("my-password")?;
    let ok = hasher.verify_password("my-password", &hash)?;
    assert!(ok);
    
    Ok(())
}
```

### Auto-Tuning Configuration
The engine can dynamically measure hardware and choose parameter costs that complete within a target duration (e.g. 300ms):
```rust
use xaor::{Xaor, XaorConfig};

let config = XaorConfig::auto_tune();
println!("Config chosen: {}", config.preview());
let hasher = Xaor::new(config)?;
```

---

## Node.js Integration (C FFI)

### Installation & Setup
1. Compile Xaor as a dynamic C-compatible library (`cdylib`):
   Ensure your `Cargo.toml` in the `xaor` repo contains:
   ```toml
   [lib]
   crate-type = ["cdylib", "rlib"]
   ```
   Compile the library:
   ```bash
   cargo build --release
   ```
2. Copy the compiled shared library output (`xaor.dll` on Windows, `libxaor.so` on Linux, `libxaor.dylib` on macOS) from `target/release/` to your Node project directory.
3. Install the FFI package:
   ```bash
   npm install ffi-napi
   ```

### Implementation Example
```javascript
const ffi = require('ffi-napi');
const path = require('path');

// Resolve path to the compiled library
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
```

---

## Python Integration (ctypes)

### Installation & Setup
1. Compile the dynamic C library (`cargo build --release`) from the `xaor` repository.
2. Locate the compiled library and copy it to a directory accessible by your Python script.
3. Load the library using Python's built-in `ctypes` module. No external pip packages are required.

### Implementation Example
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

lib.xaor_hash.argtypes = [ctypes.c_char_p]
lib.xaor_hash.restype = ctypes.c_void_p

lib.xaor_verify.argtypes = [ctypes.c_char_p, ctypes.c_char_p]
lib.xaor_verify.restype = ctypes.c_int32

lib.xaor_free_string.argtypes = [ctypes.c_void_p]
lib.xaor_free_string.restype = None

def hash_password(password: str) -> str:
    ptr = lib.xaor_hash(password.encode('utf-8'))
    if not ptr:
        raise Exception("Hashing failed")
    hash_str = ctypes.cast(ptr, ctypes.c_char_p).value.decode('utf-8')
    lib.xaor_free_string(ptr)
    return hash_str

def verify_password(password: str, hash_str: str) -> bool:
    res = lib.xaor_verify(password.encode('utf-8'), hash_str.encode('utf-8'))
    return res == 1
```

---

## Go Integration (cgo)

### Installation & Setup
1. Add the Go package module to your project:
   ```bash
   go get github.com/ogxaor/xaor/go
   ```
2. Enable `CGO_ENABLED=1` in your environment, as Go uses cgo bindings to execute the high-performance Rust cryptographic engine.
3. Ensure that the native dynamic library (`xaor.dll` / `libxaor.so` / `libxaor.dylib`) is in your system's dynamic linker search path (e.g. `/usr/local/lib` or specified via `LD_LIBRARY_PATH`).

### Implementation Example
```go
package main

import (
	"fmt"
	"log"

	"github.com/ogxaor/xaor/go"
)

func main() {
	password := "my-secure-password"

	hash, err := xaor.HashPassword(password)
	if err != nil {
		log.Fatalf("Hashing failed: %v", err)
	}
	fmt.Printf("Hash: %s\n", hash)

	ok, err := xaor.VerifyPassword(password, hash)
	if err != nil {
		log.Fatalf("Verification failed: %v", err)
	}
	fmt.Printf("Is Valid: %t\n", ok)
}
```

---

## Dart Integration (dart:ffi)

### Installation & Setup
To use Xaor in Dart projects, add the library dependency to your `pubspec.yaml`:
```yaml
dependencies:
  xaor: ^0.2.1
```
Or run the installation command:
```bash
# For Dart CLI or package development
dart pub add xaor
```
* **Dart Projects**: The package resolves and links the precompiled native library binaries into the runtime environment or dynamic linker lookup scope.


### Implementation Example
```dart
import 'package:xaor/xaor.dart';

void main() {
  final password = 'my-secure-dart-password';

  // 1. Hash password
  final hash = Xaor.hashPassword(password);
  print('Hash: $hash');

  // 2. Verify password
  final isValid = Xaor.verifyPassword(password, hash);
  print('Is Valid: $isValid');
}
```
