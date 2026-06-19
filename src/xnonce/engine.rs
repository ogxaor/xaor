use base64::{engine::general_purpose, Engine as _};
use rand::rngs::OsRng;
use rand::RngCore;

use crate::XcryptError;

#[derive(Debug, Clone)]
pub struct NonceEngine {
    length: usize,
}

impl NonceEngine {
    pub fn new(length: usize) -> Result<Self, XcryptError> {
        if length == 0 {
            return Err(XcryptError::InvalidConfig(
                "nonce length must be greater than zero".into(),
            ));
        }

        if length > 1024 {
            return Err(XcryptError::InvalidConfig(
                "nonce length must be 1024 bytes or less".into(),
            ));
        }

        Ok(Self { length })
    }

    pub fn default_96() -> Self {
        Self { length: 12 }
    }

    pub fn default_128() -> Self {
        Self { length: 16 }
    }

    pub fn generate(&self) -> Result<Vec<u8>, XcryptError> {
        let mut nonce = vec![0u8; self.length];
        OsRng
            .try_fill_bytes(&mut nonce)
            .map_err(|_| XcryptError::EntropyError)?;
        Ok(nonce)
    }

    pub fn generate_hex(&self) -> Result<String, XcryptError> {
        let nonce = self.generate()?;
        Ok(Self::encode_hex(&nonce))
    }

    pub fn generate_base64(&self) -> Result<String, XcryptError> {
        let nonce = self.generate()?;
        Ok(Self::encode_base64(&nonce))
    }

    pub fn encode_hex(bytes: &[u8]) -> String {
        to_hex(bytes)
    }

    pub fn encode_base64(bytes: &[u8]) -> String {
        general_purpose::STANDARD.encode(bytes)
    }
}

fn to_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);

    for &byte in bytes {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_correct_length() {
        let engine = NonceEngine::new(24).unwrap();
        let nonce = engine.generate().unwrap();

        assert_eq!(nonce.len(), 24);
    }

    #[test]
    fn hex_length_matches() {
        let engine = NonceEngine::new(24).unwrap();
        let nonce_hex = engine.generate_hex().unwrap();

        assert_eq!(nonce_hex.len(), 48);
        assert!(nonce_hex.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn base64_is_non_empty() {
        let engine = NonceEngine::default_128();
        let nonce_b64 = engine.generate_base64().unwrap();

        assert!(!nonce_b64.is_empty());
    }

    #[test]
    fn encodes_existing_nonce() {
        let nonce = [0x12, 0xab, 0x00, 0xff];

        assert_eq!(NonceEngine::encode_hex(&nonce), "12ab00ff");
        assert_eq!(NonceEngine::encode_base64(&nonce), "EqsA/w==");
    }
}
