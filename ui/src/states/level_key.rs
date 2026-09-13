/// Addresses one listing level of the tree: a bucket root (`prefix` is empty) or a folder
/// (`prefix` is the full key prefix, ending with '/').
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LevelKey {
    pub bucket: String,
    pub prefix: String,
}

impl LevelKey {
    pub fn bucket_root(bucket: &str) -> Self {
        Self {
            bucket: bucket.to_string(),
            prefix: String::new(),
        }
    }

    pub fn folder(bucket: &str, prefix: &str) -> Self {
        Self {
            bucket: bucket.to_string(),
            prefix: prefix.to_string(),
        }
    }

    /// The level a file is listed in: its key up to and including the last '/'.
    pub fn parent_of(bucket: &str, key: &str) -> Self {
        let prefix = match key.rfind('/') {
            Some(index) => &key[..=index],
            None => "",
        };

        Self::folder(bucket, prefix)
    }

    pub fn is_bucket_root(&self) -> bool {
        self.prefix.is_empty()
    }

    /// What is shown for this level: the bucket name for a root, the last segment for a folder.
    pub fn name(&self) -> &str {
        if self.is_bucket_root() {
            return &self.bucket;
        }

        let trimmed = self.prefix.strip_suffix('/').unwrap_or(&self.prefix);

        match trimmed.rfind('/') {
            Some(index) => &trimmed[index + 1..],
            None => trimmed,
        }
    }

    /// Every level from the bucket root down to this one, both included.
    pub fn path_from_root(&self) -> Vec<LevelKey> {
        let mut result = vec![Self::bucket_root(&self.bucket)];

        for (index, _) in self.prefix.match_indices('/') {
            result.push(Self::folder(&self.bucket, &self.prefix[..=index]));
        }

        result
    }

    /// `true` for a level strictly below `ancestor` in the same bucket, at any depth.
    pub fn is_below(&self, ancestor: &LevelKey) -> bool {
        self.bucket == ancestor.bucket
            && self.prefix.len() > ancestor.prefix.len()
            && self.prefix.starts_with(&ancestor.prefix)
    }
}
