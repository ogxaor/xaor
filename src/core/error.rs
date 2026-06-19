use thiserror::Error;

#[derive(Debug, Error)]
pub enum XaorError {
    #[error("Entropy generation failed")]
    EntropyError,

    #[error("Seed generation failed")]
    SeedError,

    #[error("Topology generation failed")]
    TopologyError,

    #[error("Compound engine failed")]
    CompoundError,

    #[error("Recycler failed")]
    RecyclerError,

    #[error("Memory arena failed")]
    MemoryError,

    #[error("Finalization failed")]
    FinalizerError,

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Verification failed")]
    VerificationFailed,

    #[error("Encryption failed")]
    EncryptionFailed,

    #[error("Decryption failed")]
    DecryptionFailed,

    #[error("Malformed stored hash: {0}")]
    MalformedStoredHash(String),

    #[error("Unsupported stored hash version: {0}")]
    UnsupportedStoredHashVersion(u8),
}

impl XaorError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::EntropyError => "ENTROPY_ERROR",
            Self::SeedError => "SEED_ERROR",
            Self::TopologyError => "TOPOLOGY_ERROR",
            Self::CompoundError => "COMPOUND_ERROR",
            Self::RecyclerError => "RECYCLER_ERROR",
            Self::MemoryError => "MEMORY_ERROR",
            Self::FinalizerError => "FINALIZER_ERROR",
            Self::InvalidConfig(_) => "INVALID_CONFIG",
            Self::VerificationFailed => "VERIFICATION_FAILED",
            Self::EncryptionFailed => "ENCRYPTION_FAILED",
            Self::DecryptionFailed => "DECRYPTION_FAILED",
            Self::MalformedStoredHash(_) => "MALFORMED_STORED_HASH",
            Self::UnsupportedStoredHashVersion(_) => "UNSUPPORTED_STORED_HASH_VERSION",
        }
    }

    pub fn catalog() -> &'static [(&'static str, &'static str)] {
        &[
            ("ENTROPY_ERROR", "Entropy generation failed"),
            ("SEED_ERROR", "Seed generation failed"),
            ("TOPOLOGY_ERROR", "Topology generation failed"),
            ("COMPOUND_ERROR", "Compound engine failed"),
            ("RECYCLER_ERROR", "Recycler failed"),
            ("MEMORY_ERROR", "Memory arena failed"),
            ("FINALIZER_ERROR", "Finalization failed"),
            ("INVALID_CONFIG", "Configuration validation failed"),
            ("VERIFICATION_FAILED", "Password verification failed"),
            ("ENCRYPTION_FAILED", "Encryption failed"),
            ("DECRYPTION_FAILED", "Decryption failed"),
            ("MALFORMED_STORED_HASH", "Stored hash string is malformed"),
            ("UNSUPPORTED_STORED_HASH_VERSION", "Stored hash version is not supported"),
        ]
    }
}
