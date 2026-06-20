import ctypes
import os
import sys

# Locate the dynamic library packaged inside the module directory
lib_dir = os.path.dirname(os.path.abspath(__file__))

if sys.platform == "win32":
    lib_name = "xaor.dll"
elif sys.platform == "darwin":
    lib_name = "libxaor.dylib"
else:
    lib_name = "libxaor.so"

lib_path = os.path.join(lib_dir, lib_name)

# If the DLL isn't packaged in the package directory (e.g. during local dev),
# fallback to target/release or target/debug
if not os.path.exists(lib_path):
    root_dir = os.path.abspath(os.path.join(lib_dir, "../../"))
    release_path = os.path.join(root_dir, "target", "release", lib_name)
    debug_path = os.path.join(root_dir, "target", "debug", lib_name)
    if os.path.exists(release_path):
        lib_path = release_path
    elif os.path.exists(debug_path):
        lib_path = debug_path
    else:
        raise FileNotFoundError(
            f"Could not locate the compiled xaor dynamic library ({lib_name}). "
            "Ensure you run 'cargo build --release' to compile it."
        )

try:
    lib = ctypes.CDLL(lib_path)
except Exception as e:
    raise ImportError(f"Failed to load shared library '{lib_path}': {e}")

# Configure FFI types
lib.xaor_hash.argtypes = [ctypes.c_char_p]
lib.xaor_hash.restype = ctypes.c_void_p  # Raw pointer to free manually

lib.xaor_verify.argtypes = [ctypes.c_char_p, ctypes.c_char_p]
lib.xaor_verify.restype = ctypes.c_int32

lib.xaor_free_string.argtypes = [ctypes.c_void_p]
lib.xaor_free_string.restype = None

lib.xaor_last_error.argtypes = []
lib.xaor_last_error.restype = ctypes.c_void_p

def hash_password(password: str) -> str:
    """Hash a password using Xaor's memory-hard pipeline."""
    if not isinstance(password, str):
        raise TypeError("Password must be a string")
        
    ptr = lib.xaor_hash(password.encode('utf-8'))
    if not ptr:
        err_ptr = lib.xaor_last_error()
        err = "Unknown cryptographic error"
        if err_ptr:
            err = ctypes.cast(err_ptr, ctypes.c_char_p).value.decode('utf-8')
            lib.xaor_free_string(err_ptr)
        raise ValueError(f"Hashing failed: {err}")
    
    hash_str = ctypes.cast(ptr, ctypes.c_char_p).value.decode('utf-8')
    lib.xaor_free_string(ptr)
    return hash_str

def verify_password(password: str, hash_str: str) -> bool:
    """Verify a password against a stored Xaor hash string."""
    if not isinstance(password, str) or not isinstance(hash_str, str):
        raise TypeError("Password and hash must be strings")
        
    res = lib.xaor_verify(password.encode('utf-8'), hash_str.encode('utf-8'))
    if res == -1:
        err_ptr = lib.xaor_last_error()
        err = "Unknown verification error"
        if err_ptr:
            err = ctypes.cast(err_ptr, ctypes.c_char_p).value.decode('utf-8')
            lib.xaor_free_string(err_ptr)
        raise ValueError(f"Verification failed: {err}")
    return res == 1
