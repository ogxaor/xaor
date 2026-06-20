use crate::core::config::{XaorConfig, XaorConfigBuilder};
use crate::core::error::XaorError;
use crate::core::engine::xaor::XaorEngine;
use subtle::ConstantTimeEq;

/// The primary Xaor interface for password hashing and verification.
///
/// # Quick Start
///
/// ```ignore
/// // One-liner — safe defaults, no config needed:
/// let hash = Xaor::quick_hash("my-password").unwrap();
/// let ok   = Xaor::quick_verify("my-password", &hash).unwrap();
///
/// // Fluent builder for custom config:
/// let xaor = Xaor::builder()
///     .memory_mb(512)
///     .rounds(24)
///     .pepper_str("my-server-secret")
///     .build()
///     .unwrap();
///
/// let hash = xaor.hash_password("my-password").unwrap();
///
/// // Auto-tune to your machine's hardware:
/// let xaor = Xaor::new(XaorConfig::auto_tune()).unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct Xaor {
    engine: XaorEngine,
}

impl Default for Xaor {
    fn default() -> Self {
        Self::new(XaorConfig::default()).expect("default config is always valid")
    }
}

impl Xaor {
    /// Create a new Xaor instance from a validated configuration.
    pub fn new(config: XaorConfig) -> Result<Self, XaorError> {
        Ok(Self {
            engine: XaorEngine::new(config)?,
        })
    }

    /// Create an Xaor instance from environment variables.
    ///
    /// Reads: `XAOR_PROFILE`, `XAOR_ROUNDS`, `XAOR_MEMORY`, `XAOR_NODES`,
    /// `XAOR_OUTPUT_SIZE`, `XAOR_MODE`, `XAOR_OUTPUT_MODE`, `XAOR_PEPPER`.
    pub fn from_env() -> Result<Self, XaorError> {
        let config = XaorConfig::from_env().map_err(XaorError::InvalidConfig)?;
        Self::new(config)
    }

    // ── Developer-Love One-Liners ─────────────────────────────────────────

    /// Hash a password using safe defaults.
    ///
    /// This is the fastest way to start using Xaor — no config needed.
    /// Equivalent to `Xaor::default().hash_password(password)`.
    ///
    /// # Example
    /// ```ignore
    /// let hash = Xaor::quick_hash("hunter2").unwrap();
    /// ```
    pub fn quick_hash(password: &str) -> Result<String, XaorError> {
        Xaor::default().hash_password(password)
    }

    /// Verify a password against a stored Xaor hash.
    ///
    /// This is the fastest way to verify — no config needed.
    /// The config parameters (rounds, memory, etc.) are read from the hash string itself.
    ///
    /// # Example
    /// ```ignore
    /// let ok = Xaor::quick_verify("hunter2", &stored_hash).unwrap();
    /// ```
    pub fn quick_verify(password: &str, stored: &str) -> Result<bool, XaorError> {
        Xaor::default().verify_password(password, stored)
    }

    /// Start a fluent builder for a custom Xaor instance.
    ///
    /// # Example
    /// ```ignore
    /// let xaor = Xaor::builder()
    ///     .memory_mb(512)
    ///     .rounds(24)
    ///     .pepper_str("server-secret")
    ///     .quantum()
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn builder() -> XaorBuilder {
        XaorBuilder::new()
    }

    // ── Core API ──────────────────────────────────────────────────────────

    /// Hash a password using the configured pipeline.
    ///
    /// Returns a self-describing hash string in the format:
    /// `$xaor$v=2$m=<MB>$r=<rounds>$n=<nodes>$o=<mode>$<salt>$<hash>`
    pub fn hash_password(&self, password: &str) -> Result<String, XaorError> {
        self.engine.hash_password(password)
    }

    /// Verify a password against a stored Xaor hash using constant-time comparison.
    pub fn verify_password(&self, password: &str, stored: &str) -> Result<bool, XaorError> {
        self.engine.verify_password(password, stored)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// XaorBuilder — Fluent Builder for Xaor instances
// ─────────────────────────────────────────────────────────────────────────────

/// Fluent builder for `Xaor` instances.
///
/// Created via `Xaor::builder()`.
pub struct XaorBuilder {
    inner: XaorConfigBuilder,
}

impl XaorBuilder {
    fn new() -> Self {
        Self { inner: XaorConfig::builder() }
    }

    pub fn rounds(mut self, rounds: usize) -> Self {
        self.inner = self.inner.rounds(rounds);
        self
    }

    pub fn memory_mb(mut self, mb: usize) -> Self {
        self.inner = self.inner.memory_mb(mb);
        self
    }

    pub fn nodes(mut self, n: usize) -> Self {
        self.inner = self.inner.nodes(n);
        self
    }

    pub fn pepper(mut self, p: Vec<u8>) -> Self {
        self.inner = self.inner.pepper(p);
        self
    }

    pub fn pepper_str(mut self, p: &str) -> Self {
        self.inner = self.inner.pepper_str(p);
        self
    }

    /// Enable 128-byte quantum-paranoid output.
    pub fn quantum(mut self) -> Self {
        self.inner = self.inner.quantum();
        self
    }

    /// Build the `Xaor` instance. Returns an error if configuration is invalid.
    pub fn build(self) -> Result<Xaor, XaorError> {
        let config = self.inner.build().map_err(XaorError::InvalidConfig)?;
        Xaor::new(config)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Utility Functions
// ─────────────────────────────────────────────────────────────────────────────

/// Return the catalog of supported Xaor error codes and short descriptions.
pub fn error_catalog() -> &'static [(&'static str, &'static str)] {
    XaorError::catalog()
}

/// Constant-time byte slice equality check.
///
/// Uses the `subtle` crate's [`ConstantTimeEq`] which inserts compiler memory
/// barriers to guarantee the comparison is **never** vectorized, branch-folded,
/// or short-circuited by LLVM — even at `opt-level = 3`. This prevents
/// timing side-channel attacks on MAC tag and hash comparisons.
///
/// # Timing Guarantee
/// Both slices must have the same length. If they do not, this function returns
/// `false` immediately (length itself is not secret). If they are the same
/// length, the comparison always takes the same number of cycles regardless
/// of where (or whether) bytes differ.
pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    // TIMING: subtle::ConstantTimeEq places a compiler barrier around the
    // XOR accumulator loop. LLVM cannot prove the result is unused and cannot
    // eliminate or short-circuit the loop. This is equivalent to the
    // `openssl_memcmp` / `crypto_memcmp` contract used in production libraries.
    a.ct_eq(b).into()
}
