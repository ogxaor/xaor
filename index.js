const ffi = require('ffi-napi');
const path = require('path');
const fs = require('fs');

// Resolve the dynamic library name based on OS
let libName;
if (process.platform === 'win32') {
  libName = 'xaor.dll';
} else if (process.platform === 'darwin') {
  libName = 'libxaor.dylib';
} else {
  libName = 'libxaor.so';
}

// Locate the dynamic library in the package directory
// For developers loading it locally, it might be in target/release or target/debug
// For production installations, it will be packaged in the root of the npm package
let libPath = path.join(__dirname, libName);

if (!fs.existsSync(libPath)) {
  const targetReleasePath = path.join(__dirname, 'target', 'release', libName);
  const targetDebugPath = path.join(__dirname, 'target', 'debug', libName);
  
  if (fs.existsSync(targetReleasePath)) {
    libPath = targetReleasePath;
  } else if (fs.existsSync(targetDebugPath)) {
    libPath = targetDebugPath;
  } else {
    throw new Error(
      `Could not locate the compiled xaor dynamic library (${libName}). ` +
      `Ensure you run "cargo build --release" to compile it before running.`
    );
  }
}

const lib = ffi.Library(libPath, {
  'xaor_hash': ['pointer', ['string']],
  'xaor_verify': ['int32', ['string', 'string']],
  'xaor_free_string': ['void', ['pointer']],
  'xaor_last_error': ['pointer', []]
});

function hashPassword(password) {
  if (typeof password !== 'string') {
    throw new TypeError('Password must be a string');
  }
  const ptr = lib.xaor_hash(password);
  if (ptr.isNull()) {
    const errPtr = lib.xaor_last_error();
    const err = errPtr.isNull() ? 'Unknown cryptographic error' : errPtr.readCString();
    if (!errPtr.isNull()) lib.xaor_free_string(errPtr);
    throw new Error('Hashing failed: ' + err);
  }
  const hash = ptr.readCString();
  lib.xaor_free_string(ptr);
  return hash;
}

function verifyPassword(password, hash) {
  if (typeof password !== 'string' || typeof hash !== 'string') {
    throw new TypeError('Password and hash must be strings');
  }
  const res = lib.xaor_verify(password, hash);
  if (res === -1) {
    const errPtr = lib.xaor_last_error();
    const err = errPtr.isNull() ? 'Unknown verification error' : errPtr.readCString();
    if (!errPtr.isNull()) lib.xaor_free_string(errPtr);
    throw new Error('Verification failed: ' + err);
  }
  return res === 1;
}

module.exports = {
  hashPassword,
  verifyPassword
};
