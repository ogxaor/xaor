package xaor

/*
#cgo CFLAGS: -I.
#cgo LDFLAGS: -L. -lxaor
#include <stdlib.h>

char* xaor_hash(const char* password);
int xaor_verify(const char* password, const char* stored);
void xaor_free_string(char* ptr);
char* xaor_last_error();
*/
import "C"
import (
	"errors"
	"unsafe"
)

// HashPassword hashes a password using Xaor's memory-hard pipeline.
func HashPassword(password string) (string, error) {
	cPassword := C.CString(password)
	defer C.free(unsafe.Pointer(cPassword))

	cHash := C.xaor_hash(cPassword)
	if cHash == nil {
		cErr := C.xaor_last_error()
		if cErr != nil {
			defer C.xaor_free_string(cErr)
			return "", errors.New(C.GoString(cErr))
		}
		return "", errors.New("unknown hashing error")
	}
	defer C.xaor_free_string(cHash)
	return C.GoString(cHash), nil
}

// VerifyPassword verifies a password against a stored Xaor hash string.
func VerifyPassword(password, hash string) (bool, error) {
	cPassword := C.CString(password)
	defer C.free(unsafe.Pointer(cPassword))

	cHash := C.CString(hash)
	defer C.free(unsafe.Pointer(cHash))

	res := C.xaor_verify(cPassword, cHash)
	if res == -1 {
		cErr := C.xaor_last_error()
		if cErr != nil {
			defer C.xaor_free_string(cErr)
			return false, errors.New(C.GoString(cErr))
		}
		return false, errors.New("unknown verification error")
	}
	return res == 1, nil
}
