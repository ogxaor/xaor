use rand::rngs::OsRng;
use rand::RngCore;

use crate::xid::config::{
    default_alphabet, validate_alphabet, validate_length, XidConfig,
};
use crate::XaorError;

#[derive(Debug, Clone)]
pub struct XidEngine {
    config: XidConfig,
}

impl XidEngine {
    pub fn new(config: XidConfig) -> Result<Self, XaorError> {
        validate_length(config.length).map_err(XaorError::InvalidConfig)?;
        validate_alphabet(&config.alphabet).map_err(XaorError::InvalidConfig)?;

        if let Some(ref prefix) = config.prefix {
            if prefix.is_empty() {
                return Err(XaorError::InvalidConfig(
                    "prefix must not be empty".into(),
                ));
            }
        }

        Ok(Self { config })
    }

    pub fn default_uuidish() -> Self {
        Self {
            config: XidConfig::default(),
        }
    }

    pub fn generate(&self) -> Result<String, XaorError> {
        let mut seed = vec![0u8; 32];
        OsRng
            .try_fill_bytes(&mut seed)
            .map_err(|_| XaorError::EntropyError)?;
        self.generate_from_seed(&seed, b"random")
    }

    pub fn generate_deterministic(&self, context: &[u8]) -> Result<String, XaorError> {
        self.generate_from_seed(context, b"deterministic")
    }

    pub fn validate(&self, value: &str) -> Result<bool, XaorError> {
        let body = self.strip_prefix(value)?;

        if body.is_empty() {
            return Ok(false);
        }

        if self.config.checksum {
            let Some((payload, checksum)) = split_checksum(body) else {
                return Ok(false);
            };

            if !self.is_valid_payload(payload) {
                return Ok(false);
            }

            return Ok(checksum == self.checksum_char(payload.as_bytes()));
        }

        Ok(self.is_valid_payload(body))
    }

    pub fn validate_checksum(&self, value: &str) -> Result<bool, XaorError> {
        if !self.config.checksum {
            return Ok(true);
        }

        let Some((body, checksum)) = split_checksum(value) else {
            return Ok(false);
        };

        let expected = self.checksum_char(body.as_bytes());
        Ok(checksum == expected)
    }

    pub fn strip_prefix<'a>(&self, value: &'a str) -> Result<&'a str, XaorError> {
        match &self.config.prefix {
            Some(prefix) => value
                .strip_prefix(prefix)
                .ok_or_else(|| XaorError::InvalidConfig("missing configured prefix".into())),
            None => Ok(value),
        }
    }

    fn generate_from_seed(&self, seed: &[u8], mode_tag: &[u8]) -> Result<String, XaorError> {
        let mut body = self.encode_seed(seed, mode_tag, self.config.length)?;

        if self.config.checksum {
            let checksum = self.checksum_char(body.as_bytes());
            body.push(checksum);
        }

        let mut output = String::new();
        if let Some(prefix) = &self.config.prefix {
            output.push_str(prefix);
        }
        output.push_str(&body);
        Ok(output)
    }

    fn encode_seed(
        &self,
        seed: &[u8],
        mode_tag: &[u8],
        length: usize,
    ) -> Result<String, XaorError> {
        let alphabet = self.config.alphabet.as_bytes();
        let alphabet_len = alphabet.len();
        let rejection_limit = (u16::MAX as usize + 1) / alphabet_len * alphabet_len;

        let mut output = String::with_capacity(length + usize::from(self.config.checksum));
        let mut counter = 0u64;
        let mut pool = Vec::new();
        let mut pool_index = 0usize;

        while output.len() < length {
            if pool_index + 1 >= pool.len() {
                pool = self.expand_pool(seed, mode_tag, counter);
                pool_index = 0;
                counter = counter.wrapping_add(1);
            }

            let value = u16::from_le_bytes([pool[pool_index], pool[pool_index + 1]]) as usize;
            pool_index += 2;

            if value >= rejection_limit {
                continue;
            }

            let idx = value % alphabet_len;
            output.push(alphabet[idx] as char);
        }

        Ok(output)
    }

    fn expand_pool(&self, seed: &[u8], mode_tag: &[u8], counter: u64) -> Vec<u8> {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"xaor.xid.v1");
        hasher.update(mode_tag);
        hasher.update(seed);
        hasher.update(&counter.to_le_bytes());
        hasher.finalize().as_bytes().to_vec()
    }

    fn checksum_char(&self, body: &[u8]) -> char {
        let alphabet = self.config.alphabet.as_bytes();
        let digest = blake3::hash(body);
        let idx = (digest.as_bytes()[0] as usize) % alphabet.len();
        alphabet[idx] as char
    }

    fn is_valid_payload(&self, value: &str) -> bool {
        value
            .chars()
            .all(|c| self.config.alphabet.as_str().contains(c))
    }
}

fn split_checksum(value: &str) -> Option<(&str, char)> {
    let mut chars = value.chars();
    let checksum = chars.next_back()?;
    let body = chars.as_str();
    Some((body, checksum))
}

impl Default for XidEngine {
    fn default() -> Self {
        Self {
            config: XidConfig {
                length: 21,
                alphabet: default_alphabet().to_string(),
                prefix: None,
                checksum: false,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic_output_matches() {
        let engine = XidEngine::new(XidConfig::default()).unwrap();
        let a = engine.generate_deterministic(b"user-123").unwrap();
        let b = engine.generate_deterministic(b"user-123").unwrap();

        assert_eq!(a, b);
    }

    #[test]
    fn different_context_changes_output() {
        let engine = XidEngine::new(XidConfig::default()).unwrap();
        let a = engine.generate_deterministic(b"user-123").unwrap();
        let b = engine.generate_deterministic(b"user-124").unwrap();

        assert_ne!(a, b);
    }

    #[test]
    fn checksum_validation_works() {
        let config = XidConfig::with_options(16, default_alphabet(), Some("xid_"), true).unwrap();
        let engine = XidEngine::new(config).unwrap();
        let id = engine.generate_deterministic(b"checksum").unwrap();

        assert!(engine.validate_checksum(engine.strip_prefix(&id).unwrap()).unwrap());
    }

    #[test]
    fn empty_input_validation_fails_without_checksum() {
        let engine = XidEngine::new(XidConfig::default()).unwrap();
        assert!(engine.validate("").is_err() || !engine.validate("").unwrap());
    }
}
