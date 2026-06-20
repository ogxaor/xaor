use std::time::Instant;

// ─────────────────────────────────────────────────────────────────────────────
// XaorMode
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum XaorMode {
    Hash,
    Encrypt,
}

impl XaorMode {
    pub fn parse(value: &str) -> Option<Self> {
        match value.to_ascii_lowercase().as_str() {
            "hash"    => Some(Self::Hash),
            "encrypt" => Some(Self::Encrypt),
            _         => None,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// XaorOutputMode
// ─────────────────────────────────────────────────────────────────────────────

/// Controls the byte length of the final hash output.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum XaorOutputMode {
    /// Standard 64-byte (512-bit) output — strong post-quantum security.
    Standard,
    /// Quantum-paranoid 128-byte (1024-bit) output using two independent
    /// BLAKE3-XOF streams. Provides 512-bit post-quantum security even
    /// against Grover + birthday attacks simultaneously.
    Quantum,
}

impl XaorOutputMode {
    pub fn byte_length(&self) -> usize {
        match self {
            Self::Standard => 64,
            Self::Quantum  => 128,
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value.to_ascii_lowercase().as_str() {
            "standard" => Some(Self::Standard),
            "quantum"  => Some(Self::Quantum),
            _          => None,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// XaorConfig
// ─────────────────────────────────────────────────────────────────────────────

/// Configuration for the Xaor cryptographic pipeline.
///
/// ## Quick Start
/// ```ignore
/// // Safe one-liner:
/// let config = XaorConfig::default();
///
/// // Fluent builder:
/// let config = XaorConfig::builder()
///     .rounds(16)
///     .memory_mb(256)
///     .pepper(b"my-server-secret".to_vec())
///     .build()
///     .unwrap();
///
/// // Automatically tuned to your machine:
/// let config = XaorConfig::auto_tune();
/// ```
#[derive(Debug, Clone)]
pub struct XaorConfig {
    /// Number of compound transformation rounds (higher = slower + stronger).
    pub rounds: usize,
    /// Memory arena size in megabytes (higher = more RAM required per hash).
    pub memory_size_mb: usize,
    /// Number of nodes in the topology graph.
    pub node_count: usize,
    /// Size of entropy vector used in Encrypt mode (bytes).
    pub output_size: usize,
    /// Hash or Encrypt mode.
    pub mode: XaorMode,
    /// Optional server-side pepper (never stored in the hash string).
    /// If set, offline cracking is impossible even after a DB leak.
    pub pepper: Option<Vec<u8>>,
    /// Output length mode: Standard (64 bytes) or Quantum (128 bytes).
    pub output_mode: XaorOutputMode,
    /// Number of parallel lanes. The memory arena and pipeline will be split
    /// across these lanes and run concurrently.
    pub lanes: usize,
}

impl Default for XaorConfig {
    fn default() -> Self {
        Self::standard()
    }
}

impl XaorConfig {
    // ── Preset Profiles ───────────────────────────────────────────────────

    /// Interactive: fast enough for login forms (~100-200ms on most machines).
    pub fn interactive() -> Self {
        Self {
            rounds: 8,
            memory_size_mb: 64,
            node_count: 16,
            output_size: 64,
            mode: XaorMode::Hash,
            pepper: None,
            output_mode: XaorOutputMode::Standard,
            lanes: 1,
        }
    }

    /// Standard: balanced security/performance for server applications (~300-500ms).
    pub fn standard() -> Self {
        Self {
            rounds: 16,
            memory_size_mb: 256,
            node_count: 32,
            output_size: 64,
            mode: XaorMode::Hash,
            pepper: None,
            output_mode: XaorOutputMode::Standard,
            lanes: 1,
        }
    }

    /// High security: for sensitive data where hash time is acceptable (~1-2s).
    pub fn high_security() -> Self {
        Self {
            rounds: 32,
            memory_size_mb: 512,
            node_count: 64,
            output_size: 64,
            mode: XaorMode::Hash,
            pepper: None,
            output_mode: XaorOutputMode::Standard,
            lanes: 1,
        }
    }

    /// Server: tuned for server-side password hashing with high concurrency.
    pub fn server() -> Self {
        Self {
            rounds: 24,
            memory_size_mb: 256,
            node_count: 48,
            output_size: 64,
            mode: XaorMode::Hash,
            pepper: None,
            output_mode: XaorOutputMode::Standard,
            lanes: 4, // Server profile assumes multithreading is available
        }
    }

    pub fn from_profile(profile: &str) -> Option<Self> {
        match profile.to_ascii_lowercase().as_str() {
            "interactive"                            => Some(Self::interactive()),
            "standard"                               => Some(Self::standard()),
            "high_security" | "high-security" | "hardened" => Some(Self::high_security()),
            "server"                                 => Some(Self::server()),
            _                                        => None,
        }
    }

    // ── Auto-Tune ─────────────────────────────────────────────────────────

    /// Automatically tune configuration to complete in approximately 300ms
    /// on the current machine.
    ///
    /// Uses a timing-based calibration:
    /// 1. Benchmarks BLAKE3 on an 8MB buffer to estimate raw throughput.
    /// 2. Scales memory and rounds to fit inside a 300ms wall-time budget.
    /// 3. Clamps to safe values (min 32MB, max 512MB) to prevent OOM.
    ///
    /// **Use this when deploying to machines you don't control** — it picks
    /// params that won't crash underpowered machines or timeout on fast ones.
    pub fn auto_tune() -> Self {
        // Step 1: Time BLAKE3 on 8MB to estimate raw throughput.
        let probe_mb = 8usize;
        let probe_buf = vec![0xABu8; probe_mb * 1024 * 1024];
        let start = Instant::now();
        let _h = blake3::hash(&probe_buf);
        let elapsed_ms = start.elapsed().as_millis().max(1);
        drop(probe_buf);

        // Step 2: Estimate safe memory budget.
        // The arena does ~3 full passes (init + forward + backward), so
        // effective cost ≈ 3x raw BLAKE3 throughput.
        let target_ms = 300u128;
        let arena_passes = 3u128;
        // Raw MB we can process in target_ms (with 3 passes overhead)
        let raw_mb_capacity = (probe_mb as u128 * target_ms) / (elapsed_ms * arena_passes);
        // Use at most 50% of what we could do (leave headroom for other stages)
        let memory_mb = (raw_mb_capacity / 2).clamp(32, 512) as usize;

        // Step 3: Use remaining time budget for rounds.
        // Each round processes the state vector (~64B) — very fast, rounds are cheap.
        // Use standard values and let memory do the heavy lifting.
        let rounds = if memory_mb >= 256 { 16 } else { 12 };
        let node_count = if memory_mb >= 128 { 32 } else { 16 };

        let cores = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1);
        let lanes = (cores / 2).clamp(1, 8); // Use up to half the cores, clamped to 1-8 lanes

        Self {
            rounds,
            memory_size_mb: memory_mb,
            node_count,
            output_size: 64,
            mode: XaorMode::Hash,
            pepper: None,
            output_mode: XaorOutputMode::Standard,
            lanes,
        }
    }

    // ── From Environment ──────────────────────────────────────────────────

    pub fn from_env() -> Result<Self, String> {
        Self::from_env_with_profile(None)
    }

    pub fn from_env_with_profile(profile_override: Option<&str>) -> Result<Self, String> {
        let mut config = profile_override
            .and_then(Self::from_profile)
            .or_else(|| {
                std::env::var("XAOR_PROFILE")
                    .ok()
                    .and_then(|p| Self::from_profile(&p))
            })
            .unwrap_or_default();

        if let Ok(v) = std::env::var("XAOR_ROUNDS") {
            config.rounds = v.parse::<usize>()
                .map_err(|_| "XAOR_ROUNDS must be a positive integer".to_string())?;
        }
        if let Ok(v) = std::env::var("XAOR_MEMORY") {
            config.memory_size_mb = v.parse::<usize>()
                .map_err(|_| "XAOR_MEMORY must be a positive integer".to_string())?;
        }
        if let Ok(v) = std::env::var("XAOR_NODES") {
            config.node_count = v.parse::<usize>()
                .map_err(|_| "XAOR_NODES must be a positive integer".to_string())?;
        }
        if let Ok(v) = std::env::var("XAOR_OUTPUT_SIZE") {
            config.output_size = v.parse::<usize>()
                .map_err(|_| "XAOR_OUTPUT_SIZE must be a positive integer".to_string())?;
        }
        if let Ok(v) = std::env::var("XAOR_MODE") {
            config.mode = XaorMode::parse(&v)
                .ok_or_else(|| "XAOR_MODE must be 'hash' or 'encrypt'".to_string())?;
        }
        if let Ok(v) = std::env::var("XAOR_OUTPUT_MODE") {
            config.output_mode = XaorOutputMode::parse(&v)
                .ok_or_else(|| "XAOR_OUTPUT_MODE must be 'standard' or 'quantum'".to_string())?;
        }
        if let Ok(v) = std::env::var("XAOR_LANES") {
            config.lanes = v.parse::<usize>()
                .map_err(|_| "XAOR_LANES must be a positive integer".to_string())?;
        }
        if let Ok(v) = std::env::var("XAOR_PEPPER") {
            config.pepper = Some(v.into_bytes());
        }

        config.validate()?;
        Ok(config)
    }

    // ── Validation ────────────────────────────────────────────────────────

    pub fn validate(&self) -> Result<(), String> {
        if self.rounds == 0 {
            return Err("rounds must be > 0".into());
        }
        if self.memory_size_mb < 8 {
            return Err(format!(
                "memory_size_mb ({}) is too small — minimum is 8 MB. \
                 Consider using XaorConfig::interactive() for low-memory machines.",
                self.memory_size_mb
            ));
        }
        if self.node_count == 0 {
            return Err("node_count must be > 0".into());
        }
        if self.output_size == 0 {
            return Err("output_size must be > 0".into());
        }
        if self.lanes == 0 {
            return Err("lanes must be > 0".into());
        }
        Ok(())
    }

    // ── Developer Utilities ───────────────────────────────────────────────

    /// Return a human-readable summary of what this configuration will do.
    pub fn preview(&self) -> String {
        format!(
            "Xaor Config:\n\
             ├─ Rounds:      {rounds}\n\
             ├─ Memory:      {mem} MB\n\
             ├─ Nodes:       {nodes}\n\
             ├─ Lanes:       {lanes}\n\
             ├─ Output mode: {out_mode}\n\
             ├─ Mode:        {mode}\n\
             └─ Pepper:      {pepper}",
            rounds   = self.rounds,
            mem      = self.memory_size_mb,
            nodes    = self.node_count,
            lanes    = self.lanes,
            out_mode = match self.output_mode {
                XaorOutputMode::Standard => "Standard (64 bytes)",
                XaorOutputMode::Quantum  => "Quantum (128 bytes)",
            },
            mode    = match self.mode {
                XaorMode::Hash    => "Hash",
                XaorMode::Encrypt => "Encrypt",
            },
            pepper  = if self.pepper.is_some() { "yes (server-side secret set)" } else { "none" },
        )
    }

    /// Start a fluent builder for XaorConfig.
    pub fn builder() -> XaorConfigBuilder {
        XaorConfigBuilder::new()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// XaorConfigBuilder — Fluent Builder API
// ─────────────────────────────────────────────────────────────────────────────

/// Fluent builder for `XaorConfig`.
///
/// # Example
/// ```ignore
/// let config = XaorConfig::builder()
///     .rounds(16)
///     .memory_mb(256)
///     .nodes(32)
///     .pepper(b"server-secret-key".to_vec())
///     .quantum()
///     .build()
///     .expect("valid config");
/// ```
pub struct XaorConfigBuilder {
    inner: XaorConfig,
}

impl XaorConfigBuilder {
    fn new() -> Self {
        Self { inner: XaorConfig::standard() }
    }

    pub fn rounds(mut self, rounds: usize) -> Self {
        self.inner.rounds = rounds;
        self
    }

    pub fn memory_mb(mut self, mb: usize) -> Self {
        self.inner.memory_size_mb = mb;
        self
    }

    pub fn nodes(mut self, n: usize) -> Self {
        self.inner.node_count = n;
        self
    }

    pub fn pepper(mut self, p: Vec<u8>) -> Self {
        self.inner.pepper = Some(p);
        self
    }

    pub fn pepper_str(mut self, p: &str) -> Self {
        self.inner.pepper = Some(p.as_bytes().to_vec());
        self
    }

    pub fn lanes(mut self, l: usize) -> Self {
        self.inner.lanes = l;
        self
    }

    /// Enable 128-byte quantum-paranoid output mode.
    pub fn quantum(mut self) -> Self {
        self.inner.output_mode = XaorOutputMode::Quantum;
        self
    }

    pub fn mode(mut self, mode: XaorMode) -> Self {
        self.inner.mode = mode;
        self
    }

    pub fn build(self) -> Result<XaorConfig, String> {
        self.inner.validate()?;
        Ok(self.inner)
    }

    /// Build without validation — for testing only.
    #[doc(hidden)]
    pub fn build_unchecked(self) -> XaorConfig {
        self.inner
    }
}
