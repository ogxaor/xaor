use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use rand::rngs::OsRng;
use rand::RngCore;

use crate::XaorError;

#[derive(Debug, Clone)]
pub struct TokenEngine {
    length: usize,
    prefix: Option<String>,
}

impl TokenEngine {
    pub fn new(length: usize) -> Result<Self, XaorError> {
        Self::with_prefix(length, None::<String>)
    }

    pub fn with_prefix<P>(length: usize, prefix: Option<P>) -> Result<Self, XaorError>
    where
        P: Into<String>,
    {
        if length == 0 {
            return Err(XaorError::InvalidConfig(
                "token length must be greater than zero".into(),
            ));
        }

        if length > 1024 {
            return Err(XaorError::InvalidConfig(
                "token length must be 1024 bytes or less".into(),
            ));
        }

        let prefix = prefix.map(Into::into);

        if let Some(ref value) = prefix {
            if value.is_empty() {
                return Err(XaorError::InvalidConfig(
                    "token prefix must not be empty".into(),
                ));
            }
        }

        Ok(Self { length, prefix })
    }

    pub fn default_session() -> Self {
        Self {
            length: 32,
            prefix: Some("xtk_".to_string()),
        }
    }

    pub fn default_api_key() -> Self {
        Self {
            length: 48,
            prefix: Some("xapi_".to_string()),
        }
    }

    pub fn generate(&self) -> Result<String, XaorError> {
        let bytes = self.generate_bytes()?;
        let encoded = Self::encode_urlsafe(&bytes);

        Ok(match &self.prefix {
            Some(prefix) => format!("{prefix}{encoded}"),
            None => encoded,
        })
    }

    pub fn generate_bytes(&self) -> Result<Vec<u8>, XaorError> {
        let mut token = vec![0u8; self.length];
        OsRng
            .try_fill_bytes(&mut token)
            .map_err(|_| XaorError::EntropyError)?;
        Ok(token)
    }

    pub fn encode_urlsafe(bytes: &[u8]) -> String {
        URL_SAFE_NO_PAD.encode(bytes)
    }

    pub fn encode_hex(bytes: &[u8]) -> String {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        let mut output = String::with_capacity(bytes.len() * 2);

        for &byte in bytes {
            output.push(HEX[(byte >> 4) as usize] as char);
            output.push(HEX[(byte & 0x0f) as usize] as char);
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_prefix_token() {
        let engine = TokenEngine::default_session();
        let token = engine.generate().unwrap();

        assert!(token.starts_with("xtk_"));
        assert!(token.len() > "xtk_".len());
    }

    #[test]
    fn generates_expected_byte_length() {
        let engine = TokenEngine::new(40).unwrap();
        let bytes = engine.generate_bytes().unwrap();

        assert_eq!(bytes.len(), 40);
    }

    #[test]
    fn encodes_existing_bytes() {
        let bytes = [0xde, 0xad, 0xbe, 0xef];

        assert_eq!(TokenEngine::encode_hex(&bytes), "deadbeef");
        assert_eq!(TokenEngine::encode_urlsafe(&bytes), "3q2-7w");
    }

    #[test]
    fn rejects_invalid_prefix() {
        let err = TokenEngine::with_prefix(16, Some("")).unwrap_err();
        assert!(err.to_string().contains("prefix"));
    }
}
