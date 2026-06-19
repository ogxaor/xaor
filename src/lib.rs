pub mod config;
pub mod error;

pub mod entropy;
pub mod seed;
pub mod topology;
pub mod compound;
pub mod recycler;
pub mod memory;
pub mod finalizer;
pub mod engine;

pub mod traits;
pub mod pipeline;
pub mod serialization;
pub mod ffi;

pub use config::{XcryptConfig, XcryptMode};
pub use error::XcryptError;
pub use serialization::StoredHash;

use engine::xcrypt::XcryptEngine;

/// High-level Xcrypt interface for password hashing and verification.
#[derive(Debug, Clone)]
pub struct Xcrypt {
    engine: XcryptEngine,
}

impl Default for Xcrypt {
    fn default() -> Self {
        Self::new(XcryptConfig::default()).expect("default config is valid")
    }
}

impl Xcrypt {
    /// Create a new Xcrypt instance from a validated configuration.
    pub fn new(config: XcryptConfig) -> Result<Self, XcryptError> {
        Ok(Self {
            engine: XcryptEngine::new(config)?,
        })
    }

    /// Hash a password using the configured pipeline.
    pub fn hash_password(&self, password: &str) -> Result<String, XcryptError> {
        self.engine.hash_password(password)
    }

    /// Verify a password against a stored Xcrypt hash.
    pub fn verify_password(&self, password: &str, stored: &str) -> Result<bool, XcryptError> {
        self.engine.verify_password(password, stored)
    }

    /// Create an Xcrypt instance from environment variables.
    pub fn from_env() -> Result<Self, XcryptError> {
        let config = XcryptConfig::from_env().map_err(XcryptError::InvalidConfig)?;
        Self::new(config)
    }
}

/// Return the catalog of supported Xcrypt error codes and short descriptions.
pub fn error_catalog() -> &'static [(&'static str, &'static str)] {
    XcryptError::catalog()
}

pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut diff = 0u8;

    for i in 0..a.len() {
        diff |= a[i] ^ b[i];
    }

    diff == 0
}
