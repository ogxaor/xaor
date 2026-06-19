use thiserror::Error;

#[derive(Debug, Error)]
pub enum XcryptError {
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
}