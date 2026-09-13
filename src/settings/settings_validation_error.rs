use super::S3ConnStringError;

#[derive(Debug, PartialEq)]
pub enum SettingsValidationError {
    NoBuckets,
    // `index` counts from 0, the message from 1.
    InvalidBucketConnString {
        index: usize,
        err: S3ConnStringError,
    },
    DuplicateBucketName {
        name: String,
    },
}

impl std::fmt::Display for SettingsValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoBuckets => write!(
                f,
                "`buckets` is empty: configure at least one bucket connection string, they are the root level of the tree"
            ),
            Self::InvalidBucketConnString { index, err } => write!(
                f,
                "bucket #{} connection string: {err}. The format is Endpoint=...;Region=...;AccessKey=...;SecretKey=...;Bucket=...",
                index + 1
            ),
            Self::DuplicateBucketName { name } => write!(
                f,
                "bucket '{name}' is configured more than once: Bucket names must be unique"
            ),
        }
    }
}
