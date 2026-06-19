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
