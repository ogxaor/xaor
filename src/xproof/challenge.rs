#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofChallenge {
    pub version: u8,
    pub subject: String,
    pub challenge_id: String,
    pub issued_at: u64,
    pub expires_at: u64,
    pub difficulty_bits: u32,
    pub salt: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofSolution {
    pub challenge_id: String,
    pub nonce: u64,
    pub hash_hex: String,
    pub leading_zero_bits: u32,
}

impl ProofChallenge {
    pub fn serialize(&self) -> String {
        format!(
            "xproof:v{}:{}:{}:{}:{}:{}:{}",
            self.version,
            self.challenge_id,
            self.subject,
            self.issued_at,
            self.expires_at,
            self.difficulty_bits,
            hex::encode(&self.salt)
        )
    }

    pub fn deserialize(serialized: &str) -> Option<Self> {
        let parts: Vec<&str> = serialized.split(':').collect();
        if parts.len() != 8 || parts[0] != "xproof" {
            return None;
        }
        let version_str = parts[1].strip_prefix('v')?;
        let version: u8 = version_str.parse().ok()?;
        let challenge_id = parts[2].to_string();
        let subject = parts[3].to_string();
        let issued_at: u64 = parts[4].parse().ok()?;
        let expires_at: u64 = parts[5].parse().ok()?;
        let difficulty_bits: u32 = parts[6].parse().ok()?;
        let salt = hex::decode(parts[7]).ok()?;

        Some(Self {
            version,
            challenge_id,
            subject,
            issued_at,
            expires_at,
            difficulty_bits,
            salt,
        })
    }
}

impl ProofSolution {
    pub fn serialize(&self) -> String {
        format!(
            "xsolution:{}:{}:{}:{}",
            self.challenge_id,
            self.nonce,
            self.hash_hex,
            self.leading_zero_bits
        )
    }

    pub fn deserialize(serialized: &str) -> Option<Self> {
        let parts: Vec<&str> = serialized.split(':').collect();
        if parts.len() != 5 || parts[0] != "xsolution" {
            return None;
        }
        let challenge_id = parts[1].to_string();
        let nonce: u64 = parts[2].parse().ok()?;
        let hash_hex = parts[3].to_string();
        let leading_zero_bits: u32 = parts[4].parse().ok()?;

        Some(Self {
            challenge_id,
            nonce,
            hash_hex,
            leading_zero_bits,
        })
    }
}
