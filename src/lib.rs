pub mod core;
pub mod xhash;
pub mod xentropy;
pub mod xchaos;
pub mod xid;
pub mod xtoken;
pub mod xnonce;
pub mod xcipher;
pub mod xvault;
pub mod xproof;
pub mod ffi;

// Re-exports for internal backwards compatibility and ease of use
pub use xentropy as entropy;
pub use xchaos as chaos;

// Re-export core modules
pub use crate::core::{
    config, error, seed, topology, compound, recycler, memory, finalizer, engine, traits,
    pipeline, serialization,
};

// ── Public API surface ────────────────────────────────────────────────────────

/// Primary hashing interface, one-liners, and constant-time utilities.
pub use xhash::{constant_time_eq, error_catalog, Xaor, XaorBuilder};

/// Configuration types.
pub use core::config::{XaorConfig, XaorConfigBuilder, XaorMode, XaorOutputMode};

/// Error type.
pub use core::error::XaorError;

/// Stored hash parser.
pub use core::serialization::StoredHash;

/// Experimental subsystems.
pub use xid::{XidConfig, XidEngine};
pub use xnonce::NonceEngine;
pub use xproof::{ProofConfig, ProofEngine};
pub use xtoken::TokenEngine;
pub use xvault::VaultEngine;
pub use xcipher::CipherEngine;
