const EXPECTED_KEYS: &str = "Endpoint, Region, AccessKey, SecretKey, Bucket and optionally Debug";

// Why a bucket connection string was refused. It never carries a credential: the connection string
// holds the secret key, and this error ends up in the start-up panic message.
#[derive(Debug, Clone, PartialEq)]
pub enum S3ConnStringError {
    // `entry` counts the `;`-separated entries from 1. The entry itself is not kept - an entry
    // without `=` may well be a secret with the key name forgotten.
    NotKeyValue { entry: usize },
    // `key` is kept only when it looks like a key name (a short ASCII word), so a fragment of a
    // secret that contained `;` is never printed.
    UnknownKey { entry: usize, key: Option<String> },
    DuplicateKey { key: &'static str },
    MissingKey { key: &'static str },
    InvalidDebugValue { value: String },
}

impl std::fmt::Display for S3ConnStringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotKeyValue { entry } => write!(f, "entry #{entry} is not Key=Value"),
            Self::UnknownKey {
                entry,
                key: Some(key),
            } => write!(
                f,
                "unknown key '{key}' (entry #{entry}), expected {EXPECTED_KEYS}"
            ),
            Self::UnknownKey { entry, key: None } => write!(
                f,
                "entry #{entry} has an unknown key, expected {EXPECTED_KEYS}"
            ),
            Self::DuplicateKey { key } => write!(f, "'{key}' is given more than once"),
            Self::MissingKey { key } => write!(f, "'{key}' is missing or empty"),
            Self::InvalidDebugValue { value } => write!(
                f,
                "'Debug' expects 1/0, true/false, yes/no or on/off - got '{value}'"
            ),
        }
    }
}
