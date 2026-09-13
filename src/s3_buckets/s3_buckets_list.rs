use crate::settings::BucketSettingsModel;

use super::S3Bucket;

// The configured buckets in settings order. Built once at start-up and never changed, so it is
// shared across requests without a lock. A handful of buckets at most: a linear lookup is enough.
pub struct S3Buckets {
    buckets: Vec<S3Bucket>,
}

impl S3Buckets {
    pub fn new(settings: &[BucketSettingsModel]) -> Self {
        Self {
            buckets: settings.iter().map(S3Bucket::new).collect(),
        }
    }

    pub fn get_all(&self) -> &[S3Bucket] {
        self.buckets.as_slice()
    }

    pub fn get(&self, name: &str) -> Option<&S3Bucket> {
        self.buckets.iter().find(|bucket| bucket.name == name)
    }
}
