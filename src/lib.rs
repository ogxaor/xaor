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

// Re-export core modules to keep all crate::<module>::... imports functioning
pub use crate::core::{
    config, error, seed, topology, compound, recycler, memory, finalizer, engine, traits,
    pipeline, serialization,
};

// Re-export public APIs
pub use core::config::{XaorConfig, XaorMode};
pub use core::error::XaorError;
pub use core::serialization::StoredHash;
pub use xhash::{constant_time_eq, error_catalog, Xaor};
pub use xid::{XidConfig, XidEngine};
pub use xnonce::NonceEngine;
pub use xproof::{ProofConfig, ProofEngine};
pub use xtoken::TokenEngine;
pub use xvault::VaultEngine;
pub use xcipher::CipherEngine;
