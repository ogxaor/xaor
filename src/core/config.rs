#[derive(Debug, Clone)]
pub enum XaorMode {
    Hash,
    Encrypt,
}

impl XaorMode {
    pub fn parse(value: &str) -> Option<Self> {
        match value.to_ascii_lowercase().as_str() {
            "hash" => Some(Self::Hash),
            "encrypt" => Some(Self::Encrypt),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct XaorConfig {
    pub rounds: usize,
    pub memory_size_mb: usize,
    pub node_count: usize,
    pub output_size: usize,
    pub mode: XaorMode,
}

impl Default for XaorConfig {
    fn default() -> Self {
        Self {
            rounds: 16,
            memory_size_mb: 256,
            node_count: 32,
            output_size: 64,
            mode: XaorMode::Hash,
        }
    }
}

impl XaorConfig {
    pub fn interactive() -> Self {
        Self {
            rounds: 8,
            memory_size_mb: 64,
            node_count: 16,
            output_size: 64,
            mode: XaorMode::Hash,
        }
    }

    pub fn standard() -> Self {
        Self {
            rounds: 16,
            memory_size_mb: 256,
            node_count: 32,
            output_size: 64,
            mode: XaorMode::Hash,
        }
    }

    pub fn high_security() -> Self {
        Self {
            rounds: 32,
            memory_size_mb: 1024,
            node_count: 64,
            output_size: 64,
            mode: XaorMode::Hash,
        }
    }

    pub fn server() -> Self {
        Self {
            rounds: 24,
            memory_size_mb: 512,
            node_count: 48,
            output_size: 64,
            mode: XaorMode::Hash,
        }
    }

    pub fn from_profile(profile: &str) -> Option<Self> {
        match profile.to_ascii_lowercase().as_str() {
            "interactive" => Some(Self::interactive()),
            "standard" => Some(Self::standard()),
            "high_security" | "high-security" | "hardened" => Some(Self::high_security()),
            "server" => Some(Self::server()),
            _ => None,
        }
    }

    pub fn from_env() -> Result<Self, String> {
        Self::from_env_with_profile(None)
    }

    pub fn from_env_with_profile(profile_override: Option<&str>) -> Result<Self, String> {
        let mut config = profile_override
            .and_then(Self::from_profile)
            .or_else(|| {
                std::env::var("XAOR_PROFILE")
                    .ok()
                    .and_then(|profile| Self::from_profile(&profile))
            })
            .unwrap_or_default();

        if let Ok(rounds) = std::env::var("XAOR_ROUNDS") {
            config.rounds = rounds
                .parse::<usize>()
                .map_err(|_| "XAOR_ROUNDS must be a positive integer".to_string())?;
        }

        if let Ok(memory) = std::env::var("XAOR_MEMORY") {
            config.memory_size_mb = memory
                .parse::<usize>()
                .map_err(|_| "XAOR_MEMORY must be a positive integer".to_string())?;
        }

        if let Ok(nodes) = std::env::var("XAOR_NODES") {
            config.node_count = nodes
                .parse::<usize>()
                .map_err(|_| "XAOR_NODES must be a positive integer".to_string())?;
        }

        if let Ok(output_size) = std::env::var("XAOR_OUTPUT_SIZE") {
            config.output_size = output_size
                .parse::<usize>()
                .map_err(|_| "XAOR_OUTPUT_SIZE must be a positive integer".to_string())?;
        }

        if let Ok(mode) = std::env::var("XAOR_MODE") {
            config.mode = XaorMode::parse(&mode)
                .ok_or_else(|| "XAOR_MODE must be either 'hash' or 'encrypt'".to_string())?;
        }

        config.validate()?;
        Ok(config)
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.rounds == 0 {
            return Err("rounds must be > 0".into());
        }

        if self.memory_size_mb < 8 {
            return Err("memory too small".into());
        }

        if self.node_count == 0 {
            return Err("node count must be > 0".into());
        }

        if self.output_size == 0 {
            return Err("output size must be > 0".into());
        }

        Ok(())
    }
}
