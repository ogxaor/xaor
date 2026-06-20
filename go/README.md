# Xaor Go Wrapper

Go FFI bindings for the **Xaor Cryptographic Engine** — an adaptive, memory-hard, chaos-keyed password hashing algorithm.

## Installation

```bash
go get github.com/ogxaor/xaor/go
```

## Setup

Ensure the compiled dynamic library (`xaor.dll`, `libxaor.so`, or `libxaor.dylib`) is in your executable directory, or system library path.

## Quick Start

```go
package main

import (
	"fmt"
	"log"

	"github.com/ogxaor/xaor/go"
)

fn main() {
	password := "my-secure-go-password"

	// 1. Hash password
	hash, err := xaor.HashPassword(password)
	if err != nil {
		log.Fatalf("Hashing failed: %v", err)
	}
	fmt.Printf("Generated Hash: %s\n", hash)

	// 2. Verify password
	isValid, err := xaor.VerifyPassword(password, hash)
	if err != nil {
		log.Fatalf("Verification failed: %v", err)
	}
	fmt.Printf("Is valid: %t\n", isValid) // true
}
```
