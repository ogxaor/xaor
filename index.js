const koffi = require('koffi');
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

const lib = koffi.load(libPath);

const xaor_hash        = lib.func('xaor_hash',        'void *', ['str']);
const xaor_verify      = lib.func('xaor_verify',      'int32',  ['str', 'str']);
const xaor_free_string = lib.func('xaor_free_string', 'void',   ['void *']);
const xaor_last_error  = lib.func('xaor_last_error',  'void *', []);

function hashPassword(password) {
  if (typeof password !== 'string') {
    throw new TypeError('Password must be a string');
  }
  const ptr = xaor_hash(password);
  if (!ptr) {
    const errPtr = xaor_last_error();
    const err = errPtr ? koffi.decode(errPtr, 'char', 256) : 'Unknown cryptographic error';
    if (errPtr) xaor_free_string(errPtr);
    throw new Error('Hashing failed: ' + err);
  }
  const hash = koffi.decode(ptr, 'char', 4096).replace(/\0.*$/, '');
  xaor_free_string(ptr);
  return hash;
}

function verifyPassword(password, hash) {
  if (typeof password !== 'string' || typeof hash !== 'string') {
    throw new TypeError('Password and hash must be strings');
  }
  const res = xaor_verify(password, hash);
  if (res === -1) {
    const errPtr = xaor_last_error();
    const err = errPtr ? koffi.decode(errPtr, 'char', 256).replace(/\0.*$/, '') : 'Unknown verification error';
    if (errPtr) xaor_free_string(errPtr);
    throw new Error('Verification failed: ' + err);
  }
  return res === 1;
}

module.exports = {
  hashPassword,
  verifyPassword,
};
