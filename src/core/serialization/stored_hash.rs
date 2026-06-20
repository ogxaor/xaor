use base64::{engine::general_purpose, Engine as _};
use std::fmt;

use crate::error::XaorError;

/// A parsed Xaor stored hash string.
///
/// ## Format (v=3)
/// ```text
/// $xaor$v=3$m=<memory_mb>$r=<rounds>$n=<nodes>$l=<lanes>$o=<output_mode>$<salt_b64>$<hash_b64>
/// ```
///
/// ## Format (v=2, legacy)
/// ```text
/// $xaor$v=2$m=<memory_mb>$r=<rounds>$n=<nodes>$o=<output_mode>$<salt_b64>$<hash_b64>
/// ```
///
/// ## Format (v=1, legacy)
/// ```text
/// $xaor$v=1$m=<memory_mb>$r=<rounds>$n=<nodes>$<salt_b64>$<hash_b64>
/// ```
///
/// The `$xaor$` prefix makes the format immediately identifiable.
/// The `v=` field enables forward-compatible upgrades.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoredHash {
    pub version: u8,
    pub memory: usize,
    pub rounds: usize,
    pub nodes: usize,
    pub lanes: usize,
    pub output_mode: String, // "std" or "qnt"
    pub salt: Vec<u8>,
    pub hash: Vec<u8>,
}

impl StoredHash {
    pub fn parse(input: &str) -> Result<Self, XaorError> {
        let parts: Vec<&str> = input.split('$').collect();

        if parts.len() < 2 || parts[1] != "xaor" {
            return Err(XaorError::MalformedStoredHash(
                "missing $xaor$ prefix".into(),
            ));
        }

        if !parts.get(2).is_some_and(|p| p.starts_with("v=")) {
            return Err(XaorError::MalformedStoredHash(
                "missing version field (expected v=1, v=2, or v=3)".into(),
            ));
        }

        let version = parts[2]
            .strip_prefix("v=")
            .ok_or_else(|| XaorError::MalformedStoredHash("invalid version field".into()))?
            .parse::<u8>()
            .map_err(|_| XaorError::MalformedStoredHash("invalid version number".into()))?;

        match version {
            1 => Self::parse_v1(&parts),
            2 => Self::parse_v2(&parts),
            3 => Self::parse_v3(&parts),
            v => Err(XaorError::UnsupportedStoredHashVersion(v)),
        }
    }

    fn parse_v1(parts: &[&str]) -> Result<Self, XaorError> {
        // v=1: $xaor$v=1$m=<>$r=<>$n=<>$<salt>$<hash>  (8 $ parts total, indices 0-7)
        if parts.len() < 8 {
            return Err(XaorError::MalformedStoredHash(
                "v=1 hash expects 8 '$'-separated fields".into(),
            ));
        }

        let memory = parse_field(parts[3], "m=", "memory")?;
        let rounds = parse_field(parts[4], "r=", "rounds")?;
        let nodes  = parse_field(parts[5], "n=", "nodes")?;

        let salt = general_purpose::STANDARD
            .decode(parts[6])
            .map_err(|_| XaorError::MalformedStoredHash("invalid salt encoding".into()))?;
        let hash = general_purpose::STANDARD
            .decode(parts[7])
            .map_err(|_| XaorError::MalformedStoredHash("invalid hash encoding".into()))?;

        Ok(Self { version: 1, memory, rounds, nodes, lanes: 1, output_mode: "std".into(), salt, hash })
    }

    fn parse_v2(parts: &[&str]) -> Result<Self, XaorError> {
        // v=2: $xaor$v=2$m=<>$r=<>$n=<>$o=<>$<salt>$<hash>  (9 $ parts, indices 0-8)
        if parts.len() < 9 {
            return Err(XaorError::MalformedStoredHash(
                "v=2 hash expects 9 '$'-separated fields".into(),
            ));
        }

        let memory = parse_field(parts[3], "m=", "memory")?;
        let rounds = parse_field(parts[4], "r=", "rounds")?;
        let nodes  = parse_field(parts[5], "n=", "nodes")?;

        let output_mode = parts[6]
            .strip_prefix("o=")
            .ok_or_else(|| XaorError::MalformedStoredHash("missing output_mode field (o=)".into()))?
            .to_string();

        let salt = general_purpose::STANDARD
            .decode(parts[7])
            .map_err(|_| XaorError::MalformedStoredHash("invalid salt encoding".into()))?;
        let hash = general_purpose::STANDARD
            .decode(parts[8])
            .map_err(|_| XaorError::MalformedStoredHash("invalid hash encoding".into()))?;

        Ok(Self { version: 2, memory, rounds, nodes, lanes: 1, output_mode, salt, hash })
    }

    fn parse_v3(parts: &[&str]) -> Result<Self, XaorError> {
        // v=3: $xaor$v=3$m=<>$r=<>$n=<>$l=<>$o=<>$<salt>$<hash>  (10 $ parts, indices 0-9)
        if parts.len() < 10 {
            return Err(XaorError::MalformedStoredHash(
                "v=3 hash expects 10 '$'-separated fields".into(),
            ));
        }

        let memory = parse_field(parts[3], "m=", "memory")?;
        let rounds = parse_field(parts[4], "r=", "rounds")?;
        let nodes  = parse_field(parts[5], "n=", "nodes")?;
        let lanes  = parse_field(parts[6], "l=", "lanes")?;

        let output_mode = parts[7]
            .strip_prefix("o=")
            .ok_or_else(|| XaorError::MalformedStoredHash("missing output_mode field (o=)".into()))?
            .to_string();

        let salt = general_purpose::STANDARD
            .decode(parts[8])
            .map_err(|_| XaorError::MalformedStoredHash("invalid salt encoding".into()))?;
        let hash = general_purpose::STANDARD
            .decode(parts[9])
            .map_err(|_| XaorError::MalformedStoredHash("invalid hash encoding".into()))?;

        Ok(Self { version: 3, memory, rounds, nodes, lanes, output_mode, salt, hash })
    }

    pub fn format_v3(&self) -> String {
        let salt_b64 = general_purpose::STANDARD.encode(&self.salt);
        let hash_b64 = general_purpose::STANDARD.encode(&self.hash);

        format!(
            "$xaor$v=3$m={}$r={}$n={}$l={}$o={}${}${}",
            self.memory, self.rounds, self.nodes, self.lanes, self.output_mode, salt_b64, hash_b64
        )
    }
}

impl fmt::Display for StoredHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.format_v3())
    }
}

fn parse_field(s: &str, prefix: &str, name: &str) -> Result<usize, XaorError> {
    s.strip_prefix(prefix)
        .ok_or_else(|| XaorError::MalformedStoredHash(format!("missing {} field ({})", name, prefix)))?
        .parse::<usize>()
        .map_err(|_| XaorError::MalformedStoredHash(format!("invalid {} value", name)))
}
