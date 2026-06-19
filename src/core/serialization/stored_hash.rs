use base64::{engine::general_purpose, Engine as _};
use std::fmt;

use crate::error::XaorError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoredHash {
    pub version: u8,
    pub memory: usize,
    pub rounds: usize,
    pub nodes: usize,
    pub salt: Vec<u8>,
    pub hash: Vec<u8>,
}

impl StoredHash {
    pub fn parse(input: &str) -> Result<Self, XaorError> {
        let parts: Vec<&str> = input.split('$').collect();

        if parts.len() < 8 {
            return Err(XaorError::MalformedStoredHash(
                "expected 8 '$'-separated fields".into(),
            ));
        }

        if parts[1] != "xaor" {
            return Err(XaorError::MalformedStoredHash(
                "missing xaor prefix".into(),
            ));
        }

        if !parts[2].starts_with("v=") {
            return Err(XaorError::MalformedStoredHash(
                "missing stored hash version".into(),
            ));
        }

        let version = parts[2]
            .strip_prefix("v=")
            .ok_or_else(|| XaorError::MalformedStoredHash("invalid version field".into()))?
            .parse::<u8>()
            .map_err(|_| XaorError::MalformedStoredHash("invalid version number".into()))?;

        if version != 1 {
            return Err(XaorError::UnsupportedStoredHashVersion(version));
        }

        let memory = parts[3]
            .strip_prefix("m=")
            .ok_or_else(|| XaorError::MalformedStoredHash("missing memory field".into()))?
            .parse::<usize>()
            .map_err(|_| XaorError::MalformedStoredHash("invalid memory value".into()))?;

        let rounds = parts[4]
            .strip_prefix("r=")
            .ok_or_else(|| XaorError::MalformedStoredHash("missing rounds field".into()))?
            .parse::<usize>()
            .map_err(|_| XaorError::MalformedStoredHash("invalid rounds value".into()))?;

        let nodes = parts[5]
            .strip_prefix("n=")
            .ok_or_else(|| XaorError::MalformedStoredHash("missing nodes field".into()))?
            .parse::<usize>()
            .map_err(|_| XaorError::MalformedStoredHash("invalid nodes value".into()))?;

        let salt = general_purpose::STANDARD
            .decode(parts[6])
            .map_err(|_| XaorError::MalformedStoredHash("invalid salt encoding".into()))?;

        let hash = general_purpose::STANDARD
            .decode(parts[7])
            .map_err(|_| XaorError::MalformedStoredHash("invalid hash encoding".into()))?;

        Ok(Self {
            version,
            memory,
            rounds,
            nodes,
            salt,
            hash,
        })
    }

    pub fn format_v1(&self) -> String {
        let salt_b64 = general_purpose::STANDARD.encode(&self.salt);
        let hash_b64 = general_purpose::STANDARD.encode(&self.hash);

        format!(
            "$xaor$v=1$m={}$r={}$n={}${}${}",
            self.memory, self.rounds, self.nodes, salt_b64, hash_b64
        )
    }
}

impl fmt::Display for StoredHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.format_v1())
    }
}
