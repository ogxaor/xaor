package main

import (
	"fmt"
	"log"

	"github.com/ogxaor/xaor/go"
)

func main() {
	fmt.Println("Testing Go FFI wrapper...")
	password := "my-secure-go-password"

	// 1. Hash password
	hash, err := xaor.HashPassword(password)
	if err != nil {
		log.Fatalf("Hashing failed: %v", err)
	}
	fmt.Printf("Generated Hash successfully: %s\n", hash)

	// 2. Verify password
	isValid, err := xaor.VerifyPassword(password, hash)
	if err != nil {
		log.Fatalf("Verification failed: %v", err)
	}
	fmt.Printf("Verification Result (should be true): %t\n", isValid)

	// 3. Verify incorrect password
	isInvalid, err := xaor.VerifyPassword("wrong-password", hash)
	if err != nil {
		log.Fatalf("Verification failed: %v", err)
	}
	fmt.Printf("Verification Result (should be false): %t\n", isInvalid)

	if isValid && !isInvalid {
		fmt.Println("OK: Go FFI wrapper test passed!")
	} else {
		log.Fatalf("ERROR: Go FFI wrapper test failed.")
	}
}
