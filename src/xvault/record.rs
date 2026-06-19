#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VaultRecord {
    pub name: String,
    pub nonce: Vec<u8>,
    pub ciphertext: Vec<u8>,
}

impl VaultRecord {
    pub fn serialize(&self) -> String {
        format!(
            "{}:{}:{}",
            self.name,
            hex::encode(&self.nonce),
            hex::encode(&self.ciphertext)
        )
    }

    pub fn parse(line: &str) -> Result<Self, String> {
        let mut parts = line.splitn(3, ':');
        let name = parts.next().ok_or_else(|| "missing name".to_string())?;
        let nonce_hex = parts.next().ok_or_else(|| "missing nonce".to_string())?;
        let ciphertext_hex = parts
            .next()
            .ok_or_else(|| "missing ciphertext".to_string())?;

        if name.is_empty() {
            return Err("name must not be empty".into());
        }

        let nonce = hex::decode(nonce_hex).map_err(|_| "invalid nonce hex".to_string())?;
        let ciphertext =
            hex::decode(ciphertext_hex).map_err(|_| "invalid ciphertext hex".to_string())?;

        Ok(Self {
            name: name.to_string(),
            nonce,
            ciphertext,
        })
    }
}
