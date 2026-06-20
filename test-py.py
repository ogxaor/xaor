import sys
sys.path.append('./python')

from xaor import hash_password, verify_password

try:
    print("Testing Python FFI wrapper...")
    password = "my-secure-python-password"
    
    print("Hashing password...")
    hashed = hash_password(password)
    print("Generated hash successfully:", hashed)
    
    print("Verifying correct password...")
    is_valid = verify_password(password, hashed)
    print("Verification result (should be True):", is_valid)
    
    print("Verifying incorrect password...")
    is_invalid = verify_password("wrong-password", hashed)
    print("Verification result (should be False):", is_invalid)
    
    if is_valid and not is_invalid:
        print("OK: Python FFI wrapper test passed!")
    else:
        print("ERROR: Python FFI wrapper test failed: incorrect validation results.")
        sys.exit(1)
except Exception as e:
    print("ERROR: Python FFI wrapper test crashed:", e)
    sys.exit(1)

