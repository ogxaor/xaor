//! Experimental proof-of-work and anti-abuse challenge engine.

use std::time::{SystemTime, UNIX_EPOCH};

use blake3::Hasher;
use rand::rngs::OsRng;
use rand::RngCore;

use crate::XcryptError;

use super::challenge::{ProofChallenge, ProofSolution};
use super::config::ProofConfig;

#[derive(Debug, Clone)]
pub struct ProofEngine {
    config: ProofConfig,
}

impl ProofEngine {
    pub fn new(config: ProofConfig) -> Result<Self, XcryptError> {
        ProofConfig::with_options(config.difficulty_bits, config.ttl_secs)
            .map_err(XcryptError::InvalidConfig)?;
        Ok(Self { config })
    }

    pub fn default_low() -> Self {
        Self {
            config: ProofConfig::low(),
        }
    }

    pub fn challenge(&self, subject: &str) -> Result<ProofChallenge, XcryptError> {
        validate_subject(subject)?;

        let issued_at = now_secs()?;
        let expires_at = issued_at.saturating_add(self.config.ttl_secs);
        let challenge_id = random_hex(16)?;
        let salt = random_bytes(32)?;

        Ok(ProofChallenge {
            version: 1,
            subject: subject.to_string(),
            challenge_id,
            issued_at,
            expires_at,
            difficulty_bits: self.config.difficulty_bits,
            salt,
        })
    }

    pub fn solve(&self, challenge: &ProofChallenge) -> Result<ProofSolution, XcryptError> {
        self.validate_challenge(challenge)?;

        let mut nonce = 0u64;
        loop {
            let hash = self.hash(challenge, nonce);
            let leading_zero_bits = count_leading_zero_bits(hash.as_bytes());

            if leading_zero_bits >= challenge.difficulty_bits {
                return Ok(ProofSolution {
                    challenge_id: challenge.challenge_id.clone(),
                    nonce,
                    hash_hex: hash.to_hex().to_string(),
                    leading_zero_bits,
                });
            }

            nonce = nonce.wrapping_add(1);
        }
    }

    pub fn verify(
        &self,
        challenge: &ProofChallenge,
        solution: &ProofSolution,
    ) -> Result<bool, XcryptError> {
        self.validate_challenge(challenge)?;

        if challenge.challenge_id != solution.challenge_id {
            return Ok(false);
        }

        let hash = self.hash(challenge, solution.nonce);
        let leading_zero_bits = count_leading_zero_bits(hash.as_bytes());

        Ok(leading_zero_bits >= challenge.difficulty_bits
            && hash.to_hex().as_str() == solution.hash_hex)
    }

    pub fn validate_challenge(&self, challenge: &ProofChallenge) -> Result<(), XcryptError> {
        if challenge.version != 1 {
            return Err(XcryptError::InvalidConfig(
                "unsupported proof challenge version".into(),
            ));
        }

        if challenge.difficulty_bits == 0 || challenge.difficulty_bits > 256 {
            return Err(XcryptError::InvalidConfig(
                "invalid proof difficulty".into(),
            ));
        }

        let now = now_secs()?;
        if now > challenge.expires_at {
            return Err(XcryptError::VerificationFailed);
        }

        Ok(())
    }

    fn hash(&self, challenge: &ProofChallenge, nonce: u64) -> blake3::Hash {
        let mut hasher = Hasher::new();
        hasher.update(b"xcrypt.xproof.v1");
        hasher.update(challenge.challenge_id.as_bytes());
        hasher.update(challenge.subject.as_bytes());
        hasher.update(&challenge.issued_at.to_le_bytes());
        hasher.update(&challenge.expires_at.to_le_bytes());
        hasher.update(&challenge.difficulty_bits.to_le_bytes());
        hasher.update(&challenge.salt);
        hasher.update(&nonce.to_le_bytes());
        hasher.finalize()
    }
}

fn now_secs() -> Result<u64, XcryptError> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .map_err(|_| XcryptError::VerificationFailed)
}

fn random_bytes(len: usize) -> Result<Vec<u8>, XcryptError> {
    let mut bytes = vec![0u8; len];
    OsRng
        .try_fill_bytes(&mut bytes)
        .map_err(|_| XcryptError::EntropyError)?;
    Ok(bytes)
}

fn random_hex(len: usize) -> Result<String, XcryptError> {
    let bytes = random_bytes(len)?;
    Ok(hex::encode(bytes))
}

fn validate_subject(subject: &str) -> Result<(), XcryptError> {
    if subject.is_empty() {
        return Err(XcryptError::InvalidConfig(
            "proof subject must not be empty".into(),
        ));
    }

    if subject.len() > 256 {
        return Err(XcryptError::InvalidConfig(
            "proof subject too long".into(),
        ));
    }

    Ok(())
}

fn count_leading_zero_bits(bytes: &[u8]) -> u32 {
    let mut total = 0u32;

    for byte in bytes {
        if *byte == 0 {
            total += 8;
            continue;
        }

        total += byte.leading_zeros();
        break;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge_solves_and_verifies() {
        let engine = ProofEngine::new(ProofConfig::low()).unwrap();
        let challenge = engine.challenge("login").unwrap();
        let solution = engine.solve(&challenge).unwrap();

        assert!(engine.verify(&challenge, &solution).unwrap());
    }

    #[test]
    fn deterministic_verification_fails_for_wrong_nonce() {
        let engine = ProofEngine::new(ProofConfig::low()).unwrap();
        let challenge = engine.challenge("login").unwrap();
        let mut solution = engine.solve(&challenge).unwrap();
        solution.nonce = solution.nonce.wrapping_add(1);

        assert!(!engine.verify(&challenge, &solution).unwrap());
    }

    #[test]
    fn invalid_subject_rejected() {
        let engine = ProofEngine::default_low();
        assert!(engine.challenge("").is_err());
    }
}
