use std::cell::RefCell;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

use crate::Xcrypt;

thread_local! {
    static LAST_ERROR: RefCell<Option<String>> = const { RefCell::new(None) };
}

fn set_last_error(message: impl Into<String>) {
    LAST_ERROR.with(|slot| {
        *slot.borrow_mut() = Some(message.into().replace('\0', " "));
    });
}

fn clear_last_error() {
    LAST_ERROR.with(|slot| {
        *slot.borrow_mut() = None;
    });
}

fn c_str_from_ptr<'a>(ptr: *const c_char) -> Result<&'a CStr, String> {
    if ptr.is_null() {
        return Err("null pointer provided".into());
    }

    unsafe { CStr::from_ptr(ptr) }
        .to_str()
        .map_err(|_| "input was not valid UTF-8".to_string())?;

    Ok(unsafe { CStr::from_ptr(ptr) })
}

#[no_mangle]
pub extern "C" fn xcrypt_last_error() -> *mut c_char {
    LAST_ERROR.with(|slot| match slot.borrow().as_ref() {
        Some(err) => CString::new(err.as_str())
            .map(|s| s.into_raw())
            .unwrap_or(ptr::null_mut()),
        None => ptr::null_mut(),
    })
}

#[no_mangle]
pub extern "C" fn xcrypt_free_string(_ptr: *mut c_char) {
    if !_ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(_ptr);
        }
    }
}

#[no_mangle]
pub extern "C" fn xcrypt_free_bytes(ptr: *mut u8, len: usize) {
    if !ptr.is_null() {
        unsafe {
            let _ = Vec::from_raw_parts(ptr, len, len);
        }
    }
}

#[no_mangle]
pub extern "C" fn xcrypt_hash(password: *const c_char) -> *mut c_char {
    clear_last_error();

    let password = match c_str_from_ptr(password).and_then(|s| {
        s.to_str()
            .map_err(|_| "password was not valid UTF-8".to_string())
    }) {
        Ok(password) => password,
        Err(err) => {
            set_last_error(err);
            return ptr::null_mut();
        }
    };

    let engine = match Xcrypt::from_env() {
        Ok(engine) => engine,
        Err(err) => {
            set_last_error(format!("{} ({})", err, err.code()));
            return ptr::null_mut();
        }
    };

    match engine.hash_password(password) {
        Ok(hash) => match CString::new(hash) {
            Ok(c_string) => c_string.into_raw(),
            Err(_) => {
                set_last_error("generated hash contained NUL byte");
                ptr::null_mut()
            }
        },
        Err(err) => {
            set_last_error(format!("{} ({})", err, err.code()));
            ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn xcrypt_verify(
    password: *const c_char,
    stored: *const c_char,
) -> i32 {
    clear_last_error();

    let password = match c_str_from_ptr(password).and_then(|s| {
        s.to_str()
            .map_err(|_| "password was not valid UTF-8".to_string())
    }) {
        Ok(password) => password,
        Err(err) => {
            set_last_error(err);
            return -1;
        }
    };

    let stored = match c_str_from_ptr(stored).and_then(|s| {
        s.to_str()
            .map_err(|_| "stored hash was not valid UTF-8".to_string())
    }) {
        Ok(stored) => stored,
        Err(err) => {
            set_last_error(err);
            return -1;
        }
    };

    let engine = match Xcrypt::from_env() {
        Ok(engine) => engine,
        Err(err) => {
            set_last_error(format!("{} ({})", err, err.code()));
            return -1;
        }
    };

    match engine.verify_password(password, stored) {
        Ok(true) => 1,
        Ok(false) => 0,
        Err(err) => {
            set_last_error(format!("{} ({})", err, err.code()));
            -1
        }
    }
}

// Subsystems FFI Expositions

#[no_mangle]
pub extern "C" fn xcrypt_xnonce_generate(length: usize) -> *mut c_char {
    clear_last_error();
    match crate::NonceEngine::new(length).and_then(|e| e.generate_hex()) {
        Ok(nonce_hex) => match CString::new(nonce_hex) {
            Ok(c_str) => c_str.into_raw(),
            Err(_) => {
                set_last_error("nonce hex contained NUL byte");
                ptr::null_mut()
            }
        },
        Err(err) => {
            set_last_error(format!("{} ({})", err, err.code()));
            ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn xcrypt_xtoken_generate(length: usize, prefix: *const c_char) -> *mut c_char {
    clear_last_error();
    let prefix_str = if prefix.is_null() {
        None
    } else {
        match unsafe { CStr::from_ptr(prefix) }.to_str() {
            Ok(s) => Some(s),
            Err(err) => {
                set_last_error(format!("invalid prefix UTF-8: {}", err));
                return ptr::null_mut();
            }
        }
    };

    match crate::TokenEngine::with_prefix(length, prefix_str).and_then(|e| e.generate()) {
        Ok(token) => match CString::new(token) {
            Ok(c_str) => c_str.into_raw(),
            Err(_) => {
                set_last_error("token contained NUL byte");
                ptr::null_mut()
            }
        },
        Err(err) => {
            set_last_error(format!("{} ({})", err, err.code()));
            ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn xcrypt_xid_generate(
    length: usize,
    prefix: *const c_char,
    checksum: i32,
) -> *mut c_char {
    clear_last_error();
    let prefix_str = if prefix.is_null() {
        None
    } else {
        match unsafe { CStr::from_ptr(prefix) }.to_str() {
            Ok(s) => Some(s),
            Err(err) => {
                set_last_error(format!("invalid prefix UTF-8: {}", err));
                return ptr::null_mut();
            }
        }
    };

    let config = match crate::XidConfig::with_options(
        length,
        crate::xid::config::default_alphabet().to_string(),
        prefix_str.map(|s| s.to_string()),
        checksum != 0,
    ) {
        Ok(cfg) => cfg,
        Err(err) => {
            set_last_error(format!("invalid configuration: {}", err));
            return ptr::null_mut();
        }
    };

    match crate::XidEngine::new(config).and_then(|e| e.generate()) {
        Ok(id) => match CString::new(id) {
            Ok(c_str) => c_str.into_raw(),
            Err(_) => {
                set_last_error("xid contained NUL byte");
                ptr::null_mut()
            }
        },
        Err(err) => {
            set_last_error(format!("{} ({})", err, err.code()));
            ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn xcrypt_xid_validate(
    id: *const c_char,
    prefix: *const c_char,
    checksum: i32,
) -> i32 {
    clear_last_error();
    let id_str = match c_str_from_ptr(id) {
        Ok(s) => match s.to_str() {
            Ok(val) => val,
            Err(_) => return 0,
        },
        Err(_) => return 0,
    };

    let prefix_str = if prefix.is_null() {
        None
    } else {
        match unsafe { CStr::from_ptr(prefix) }.to_str() {
            Ok(s) => Some(s),
            Err(_) => return 0,
        }
    };

    let config = match crate::XidConfig::with_options(
        21,
        crate::xid::config::default_alphabet().to_string(),
        prefix_str.map(|s| s.to_string()),
        checksum != 0,
    ) {
        Ok(cfg) => cfg,
        Err(_) => return 0,
    };

    let engine = match crate::XidEngine::new(config) {
        Ok(eng) => eng,
        Err(_) => return 0,
    };

    match engine.validate(id_str) {
        Ok(true) => 1,
        _ => 0,
    }
}

#[no_mangle]
pub extern "C" fn xcrypt_xcipher_encrypt(
    plaintext: *const u8,
    plaintext_len: usize,
    key: *const u8,
    key_len: usize,
    nonce: *const u8,
    nonce_len: usize,
    ad: *const u8,
    ad_len: usize,
    out_len: *mut usize,
) -> *mut u8 {
    clear_last_error();
    if plaintext.is_null() || key.is_null() || nonce.is_null() || out_len.is_null() {
        set_last_error("null pointer provided");
        return ptr::null_mut();
    }

    let plaintext_slice = unsafe { std::slice::from_raw_parts(plaintext, plaintext_len) };
    let key_slice = unsafe { std::slice::from_raw_parts(key, key_len) };
    let nonce_slice = unsafe { std::slice::from_raw_parts(nonce, nonce_len) };
    let ad_slice = if ad.is_null() {
        &[]
    } else {
        unsafe { std::slice::from_raw_parts(ad, ad_len) }
    };

    let engine = crate::CipherEngine::new();
    match engine.encrypt(plaintext_slice, key_slice, nonce_slice, ad_slice) {
        Ok(encrypted) => {
            let mut encrypted_vec = encrypted.into_boxed_slice();
            let len = encrypted_vec.len();
            let res_ptr = encrypted_vec.as_mut_ptr();
            std::mem::forget(encrypted_vec);
            unsafe {
                *out_len = len;
            }
            res_ptr
        }
        Err(err) => {
            set_last_error(format!("{} ({})", err, err.code()));
            ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn xcrypt_xcipher_decrypt(
    ciphertext: *const u8,
    ciphertext_len: usize,
    key: *const u8,
    key_len: usize,
    nonce: *const u8,
    nonce_len: usize,
    ad: *const u8,
    ad_len: usize,
    out_len: *mut usize,
) -> *mut u8 {
    clear_last_error();
    if ciphertext.is_null() || key.is_null() || nonce.is_null() || out_len.is_null() {
        set_last_error("null pointer provided");
        return ptr::null_mut();
    }

    let ciphertext_slice = unsafe { std::slice::from_raw_parts(ciphertext, ciphertext_len) };
    let key_slice = unsafe { std::slice::from_raw_parts(key, key_len) };
    let nonce_slice = unsafe { std::slice::from_raw_parts(nonce, nonce_len) };
    let ad_slice = if ad.is_null() {
        &[]
    } else {
        unsafe { std::slice::from_raw_parts(ad, ad_len) }
    };

    let engine = crate::CipherEngine::new();
    match engine.decrypt(ciphertext_slice, key_slice, nonce_slice, ad_slice) {
        Ok(decrypted) => {
            let mut decrypted_vec = decrypted.into_boxed_slice();
            let len = decrypted_vec.len();
            let res_ptr = decrypted_vec.as_mut_ptr();
            std::mem::forget(decrypted_vec);
            unsafe {
                *out_len = len;
            }
            res_ptr
        }
        Err(err) => {
            set_last_error(format!("{} ({})", err, err.code()));
            ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn xcrypt_xvault_store(
    path: *const c_char,
    name: *const c_char,
    secret: *const u8,
    secret_len: usize,
    master_key: *const u8,
    master_key_len: usize,
) -> i32 {
    clear_last_error();
    let path_str = match c_str_from_ptr(path) {
        Ok(s) => match s.to_str() {
            Ok(val) => val,
            Err(err) => {
                set_last_error(format!("invalid path UTF-8: {}", err));
                return -1;
            }
        },
        Err(err) => {
            set_last_error(err);
            return -1;
        }
    };

    let name_str = match c_str_from_ptr(name) {
        Ok(s) => match s.to_str() {
            Ok(val) => val,
            Err(err) => {
                set_last_error(format!("invalid name UTF-8: {}", err));
                return -1;
            }
        },
        Err(err) => {
            set_last_error(err);
            return -1;
        }
    };

    if secret.is_null() || master_key.is_null() {
        set_last_error("null secret or master key pointer");
        return -1;
    }

    let secret_slice = unsafe { std::slice::from_raw_parts(secret, secret_len) };
    let key_slice = unsafe { std::slice::from_raw_parts(master_key, master_key_len) };

    let engine = crate::VaultEngine::new(path_str);
    match engine.store(name_str, secret_slice, key_slice) {
        Ok(_) => 1,
        Err(err) => {
            set_last_error(format!("{} ({})", err, err.code()));
            -1
        }
    }
}

#[no_mangle]
pub extern "C" fn xcrypt_xvault_retrieve(
    path: *const c_char,
    name: *const c_char,
    master_key: *const u8,
    master_key_len: usize,
    out_len: *mut usize,
) -> *mut u8 {
    clear_last_error();
    if out_len.is_null() {
        set_last_error("null out_len pointer");
        return ptr::null_mut();
    }

    let path_str = match c_str_from_ptr(path) {
        Ok(s) => match s.to_str() {
            Ok(val) => val,
            Err(err) => {
                set_last_error(format!("invalid path UTF-8: {}", err));
                return ptr::null_mut();
            }
        },
        Err(err) => {
            set_last_error(err);
            return ptr::null_mut();
        }
    };

    let name_str = match c_str_from_ptr(name) {
        Ok(s) => match s.to_str() {
            Ok(val) => val,
            Err(err) => {
                set_last_error(format!("invalid name UTF-8: {}", err));
                return ptr::null_mut();
            }
        },
        Err(err) => {
            set_last_error(err);
            return ptr::null_mut();
        }
    };

    if master_key.is_null() {
        set_last_error("null master key pointer");
        return ptr::null_mut();
    }

    let key_slice = unsafe { std::slice::from_raw_parts(master_key, master_key_len) };

    let engine = crate::VaultEngine::new(path_str);
    match engine.retrieve(name_str, key_slice) {
        Ok(Some(secret)) => {
            let mut secret_vec = secret.into_boxed_slice();
            let len = secret_vec.len();
            let res_ptr = secret_vec.as_mut_ptr();
            std::mem::forget(secret_vec);
            unsafe {
                *out_len = len;
            }
            res_ptr
        }
        Ok(None) => {
            unsafe {
                *out_len = 0;
            }
            ptr::null_mut()
        }
        Err(err) => {
            set_last_error(format!("{} ({})", err, err.code()));
            ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn xcrypt_xproof_challenge(
    subject: *const c_char,
    difficulty: u32,
) -> *mut c_char {
    clear_last_error();
    let subject_str = match c_str_from_ptr(subject) {
        Ok(s) => match s.to_str() {
            Ok(val) => val,
            Err(err) => {
                set_last_error(format!("invalid subject UTF-8: {}", err));
                return ptr::null_mut();
            }
        },
        Err(err) => {
            set_last_error(err);
            return ptr::null_mut();
        }
    };

    let config = crate::ProofConfig {
        difficulty_bits: difficulty,
        ttl_secs: 300,
    };

    let engine = match crate::ProofEngine::new(config) {
        Ok(eng) => eng,
        Err(err) => {
            set_last_error(format!("invalid configuration: {}", err));
            return ptr::null_mut();
        }
    };

    match engine.challenge(subject_str) {
        Ok(challenge) => {
            let serialized = challenge.serialize();
            match CString::new(serialized) {
                Ok(c_str) => c_str.into_raw(),
                Err(_) => {
                    set_last_error("serialized challenge contained NUL byte");
                    ptr::null_mut()
                }
            }
        }
        Err(err) => {
            set_last_error(format!("{} ({})", err, err.code()));
            ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn xcrypt_xproof_solve(challenge_serialized: *const c_char) -> *mut c_char {
    clear_last_error();
    let challenge_str = match c_str_from_ptr(challenge_serialized) {
        Ok(s) => match s.to_str() {
            Ok(val) => val,
            Err(err) => {
                set_last_error(format!("invalid challenge UTF-8: {}", err));
                return ptr::null_mut();
            }
        },
        Err(err) => {
            set_last_error(err);
            return ptr::null_mut();
        }
    };

    let challenge = match crate::xproof::ProofChallenge::deserialize(challenge_str) {
        Some(ch) => ch,
        None => {
            set_last_error("deserialization error");
            return ptr::null_mut();
        }
    };

    let engine = crate::ProofEngine::default_low();
    match engine.solve(&challenge) {
        Ok(solution) => {
            let serialized = solution.serialize();
            match CString::new(serialized) {
                Ok(c_str) => c_str.into_raw(),
                Err(_) => {
                    set_last_error("serialized solution contained NUL byte");
                    ptr::null_mut()
                }
            }
        }
        Err(err) => {
            set_last_error(format!("{} ({})", err, err.code()));
            ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn xcrypt_xproof_verify(
    challenge_serialized: *const c_char,
    solution_serialized: *const c_char,
) -> i32 {
    clear_last_error();
    let challenge_str = match c_str_from_ptr(challenge_serialized) {
        Ok(s) => match s.to_str() {
            Ok(val) => val,
            Err(err) => {
                set_last_error(format!("invalid challenge UTF-8: {}", err));
                return -1;
            }
        },
        Err(err) => {
            set_last_error(err);
            return -1;
        }
    };

    let solution_str = match c_str_from_ptr(solution_serialized) {
        Ok(s) => match s.to_str() {
            Ok(val) => val,
            Err(err) => {
                set_last_error(format!("invalid solution UTF-8: {}", err));
                return -1;
            }
        },
        Err(err) => {
            set_last_error(err);
            return -1;
        }
    };

    let challenge = match crate::xproof::ProofChallenge::deserialize(challenge_str) {
        Some(ch) => ch,
        None => return 0,
    };

    let solution = match crate::xproof::ProofSolution::deserialize(solution_str) {
        Some(sol) => sol,
        None => return 0,
    };

    let engine = crate::ProofEngine::default_low();
    match engine.verify(&challenge, &solution) {
        Ok(true) => 1,
        _ => 0,
    }
}
