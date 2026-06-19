//! Experimental local vault for storing secrets at rest.

use std::fs;
use std::path::{Path, PathBuf};

use blake3::Hasher;
use rand::rngs::OsRng;
use rand::RngCore;

use crate::XcryptError;

use super::record::VaultRecord;

#[derive(Debug, Clone)]
pub struct VaultEngine {
    path: PathBuf,
}

impl VaultEngine {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn store(
        &self,
        name: &str,
        secret: &[u8],
        master_key: &[u8],
    ) -> Result<(), XcryptError> {
        validate_name(name)?;
        validate_master_key(master_key)?;

        let nonce = random_nonce(32)?;
        let ciphertext = xor_crypt(secret, master_key, &nonce);
        let record = VaultRecord {
            name: name.to_string(),
            nonce,
            ciphertext,
        };

        let mut records = self.load_all_records()?;
        records.retain(|existing| existing.name != name);
        records.push(record);

        self.write_records(&records)?;
        Ok(())
    }

    pub fn retrieve(
        &self,
        name: &str,
        master_key: &[u8],
    ) -> Result<Option<Vec<u8>>, XcryptError> {
        validate_name(name)?;
        validate_master_key(master_key)?;

        let records = self.load_all_records()?;
        let Some(record) = records.into_iter().find(|record| record.name == name) else {
            return Ok(None);
        };

        Ok(Some(xor_crypt(&record.ciphertext, master_key, &record.nonce)))
    }

    pub fn delete(&self, name: &str) -> Result<bool, XcryptError> {
        validate_name(name)?;

        let mut records = self.load_all_records()?;
        let before = records.len();
        records.retain(|existing| existing.name != name);

        if records.len() == before {
            return Ok(false);
        }

        self.write_records(&records)?;
        Ok(true)
    }

    pub fn list(&self) -> Result<Vec<String>, XcryptError> {
        Ok(self
            .load_all_records()?
            .into_iter()
            .map(|record| record.name)
            .collect())
    }

    pub fn exists(&self, name: &str) -> Result<bool, XcryptError> {
        validate_name(name)?;
        Ok(self.list()?.iter().any(|entry| entry == name))
    }

    fn load_all_records(&self) -> Result<Vec<VaultRecord>, XcryptError> {
        if !self.path.exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(&self.path)
            .map_err(|_| XcryptError::EncryptionFailed)?;

        let mut records = Vec::new();

        for line in content.lines() {
            if line.trim().is_empty() {
                continue;
            }

            let record = VaultRecord::parse(line)
                .map_err(|_| XcryptError::MalformedStoredHash("invalid vault record".into()))?;
            records.push(record);
        }

        Ok(records)
    }

    fn write_records(&self, records: &[VaultRecord]) -> Result<(), XcryptError> {
        if let Some(parent) = self.path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent).map_err(|_| XcryptError::EncryptionFailed)?;
            }
        }

        let serialized = records
            .iter()
            .map(VaultRecord::serialize)
            .collect::<Vec<_>>()
            .join("\n");

        fs::write(&self.path, serialized).map_err(|_| XcryptError::EncryptionFailed)?;
        Ok(())
    }
}

fn validate_name(name: &str) -> Result<(), XcryptError> {
    if name.is_empty() {
        return Err(XcryptError::InvalidConfig(
            "vault name must not be empty".into(),
        ));
    }

    if name.chars().any(|c| c == ':' || c == '\n' || c == '\r') {
        return Err(XcryptError::InvalidConfig(
            "vault name contains unsupported characters".into(),
        ));
    }

    Ok(())
}

fn validate_master_key(master_key: &[u8]) -> Result<(), XcryptError> {
    if master_key.len() < 16 {
        return Err(XcryptError::InvalidConfig(
            "master key must be at least 16 bytes".into(),
        ));
    }

    Ok(())
}

fn random_nonce(length: usize) -> Result<Vec<u8>, XcryptError> {
    let mut nonce = vec![0u8; length];
    OsRng
        .try_fill_bytes(&mut nonce)
        .map_err(|_| XcryptError::EntropyError)?;
    Ok(nonce)
}

fn xor_crypt(input: &[u8], master_key: &[u8], nonce: &[u8]) -> Vec<u8> {
    let mut output = Vec::with_capacity(input.len());
    let mut counter = 0u64;
    let mut keystream = derive_keystream(master_key, nonce, counter);
    let mut stream_index = 0usize;

    for &byte in input {
        if stream_index >= keystream.len() {
            counter = counter.wrapping_add(1);
            keystream = derive_keystream(master_key, nonce, counter);
            stream_index = 0;
        }

        output.push(byte ^ keystream[stream_index]);
        stream_index += 1;
    }

    output
}

fn derive_keystream(master_key: &[u8], nonce: &[u8], counter: u64) -> Vec<u8> {
    let mut hasher = Hasher::new();
    hasher.update(b"xcrypt.xvault.v1");
    hasher.update(master_key);
    hasher.update(nonce);
    hasher.update(&counter.to_le_bytes());
    let mut stream = Vec::with_capacity(32);
    stream.extend_from_slice(hasher.finalize().as_bytes());
    stream
}

impl Default for VaultEngine {
    fn default() -> Self {
        Self::new("xcrypt.vault")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_path(name: &str) -> PathBuf {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("xcrypt-{name}-{stamp}.vault"))
    }

    #[test]
    fn store_and_retrieve_secret() {
        let path = temp_path("store");
        let vault = VaultEngine::new(&path);
        let key = b"0123456789abcdef0123456789abcdef";

        vault.store("db_password", b"super-secret", key).unwrap();
        let retrieved = vault.retrieve("db_password", key).unwrap().unwrap();

        assert_eq!(retrieved, b"super-secret");
        let _ = fs::remove_file(path);
    }

    #[test]
    fn list_and_delete_secret() {
        let path = temp_path("list");
        let vault = VaultEngine::new(&path);
        let key = b"0123456789abcdef0123456789abcdef";

        vault.store("api_key", b"secret-1", key).unwrap();
        vault.store("session", b"secret-2", key).unwrap();

        let names = vault.list().unwrap();
        assert!(names.contains(&"api_key".to_string()));
        assert!(names.contains(&"session".to_string()));

        assert!(vault.delete("api_key").unwrap());
        assert!(!vault.exists("api_key").unwrap());
        let _ = fs::remove_file(path);
    }

    #[test]
    fn invalid_name_is_rejected() {
        let vault = VaultEngine::default();
        let key = b"0123456789abcdef0123456789abcdef";

        let err = vault.store("bad:name", b"secret", key).unwrap_err();
        assert!(err.to_string().contains("vault name"));
    }
}
