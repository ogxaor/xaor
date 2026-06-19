#[derive(Debug, Clone)]
pub struct XidConfig {
    pub length: usize,
    pub alphabet: String,
    pub prefix: Option<String>,
    pub checksum: bool,
}

impl Default for XidConfig {
    fn default() -> Self {
        Self {
            length: 21,
            alphabet: default_alphabet().to_string(),
            prefix: None,
            checksum: false,
        }
    }
}

impl XidConfig {
    pub fn new(length: usize, alphabet: impl Into<String>) -> Result<Self, String> {
        Self::with_options(length, alphabet, None::<String>, false)
    }

    pub fn with_options<P>(
        length: usize,
        alphabet: impl Into<String>,
        prefix: Option<P>,
        checksum: bool,
    ) -> Result<Self, String>
    where
        P: Into<String>,
    {
        let alphabet = alphabet.into();
        validate_length(length)?;
        validate_alphabet(&alphabet)?;

        let prefix = prefix.map(Into::into);
        if let Some(ref value) = prefix {
            if value.is_empty() {
                return Err("prefix must not be empty".into());
            }
        }

        Ok(Self {
            length,
            alphabet,
            prefix,
            checksum,
        })
    }

    pub fn with_prefix(
        length: usize,
        alphabet: impl Into<String>,
        prefix: impl Into<String>,
    ) -> Result<Self, String> {
        Self::with_options(length, alphabet, Some(prefix), false)
    }

    pub fn deterministic(
        length: usize,
        alphabet: impl Into<String>,
    ) -> Result<Self, String> {
        Self::with_options(length, alphabet, None::<String>, true)
    }
}

pub fn default_alphabet() -> &'static str {
    "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
}

pub fn validate_length(length: usize) -> Result<(), String> {
    if length == 0 {
        return Err("xid length must be greater than zero".into());
    }

    if length > 128 {
        return Err("xid length must be 128 characters or less".into());
    }

    Ok(())
}

pub fn validate_alphabet(alphabet: &str) -> Result<(), String> {
    if alphabet.len() < 2 {
        return Err("alphabet must contain at least 2 characters".into());
    }

    if !alphabet.is_ascii() {
        return Err("alphabet must be ASCII".into());
    }

    let mut seen = [false; 256];
    for byte in alphabet.bytes() {
        let index = byte as usize;
        if seen[index] {
            return Err("alphabet must not contain duplicate characters".into());
        }
        seen[index] = true;
    }

    Ok(())
}
