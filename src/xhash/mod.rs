use crate::core::config::XaorConfig;
use crate::core::error::XaorError;
use crate::core::engine::xaor::XaorEngine;

/// High-level Xaor interface for password hashing and verification.
#[derive(Debug, Clone)]
pub struct Xaor {
    engine: XaorEngine,
}

impl Default for Xaor {
    fn default() -> Self {
        Self::new(XaorConfig::default()).expect("default config is valid")
    }
}

impl Xaor {
    /// Create a new Xaor instance from a validated configuration.
    pub fn new(config: XaorConfig) -> Result<Self, XaorError> {
        Ok(Self {
            engine: XaorEngine::new(config)?,
        })
    }

    /// Hash a password using the configured pipeline.
    pub fn hash_password(&self, password: &str) -> Result<String, XaorError> {
        self.engine.hash_password(password)
    }

    /// Verify a password against a stored Xaor hash.
    pub fn verify_password(&self, password: &str, stored: &str) -> Result<bool, XaorError> {
        self.engine.verify_password(password, stored)
    }

    /// Create an Xaor instance from environment variables.
    pub fn from_env() -> Result<Self, XaorError> {
        let config = XaorConfig::from_env().map_err(XaorError::InvalidConfig)?;
        Self::new(config)
    }
}

/// Return the catalog of supported Xaor error codes and short descriptions.
pub fn error_catalog() -> &'static [(&'static str, &'static str)] {
    XaorError::catalog()
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
