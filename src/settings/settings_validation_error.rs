#[derive(Debug, PartialEq)]
pub enum SettingsValidationError {
    NoBuckets,
    EmptyBucketName { index: usize },
    DuplicateBucketName { name: String },
}

impl std::fmt::Display for SettingsValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoBuckets => write!(
                f,
                "`buckets` is empty: configure at least one bucket, they are the root level of the tree"
            ),
            Self::EmptyBucketName { index } => {
                write!(f, "bucket #{} has an empty `name`", index + 1)
            }
            Self::DuplicateBucketName { name } => write!(
                f,
                "bucket name '{name}' is configured more than once: names must be unique"
            ),
        }
    }
}
