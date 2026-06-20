# Language Bindings

---

## Rust Integration

For Rust applications, use Xaor directly as a dependency. The API supports a simple one-liner interface and a customization builder:

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

If you are calling Xaor's compiled shared library (`.dll` on Windows, `.so` on Linux, `.dylib` on macOS) from Node.js, you can use the `ffi-napi` package:

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

## Python Integration (`ctypes`)

In Python, the standard library provides `ctypes` which lets you load and execute compiled FFI binaries without installing any packages:

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

## Go Integration (`cgo`)

Import the official Go wrapper package and call the hashing routines directly:

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
