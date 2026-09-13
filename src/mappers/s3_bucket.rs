use rest_api_shared::BucketHttpModel;

use crate::s3_buckets::S3Bucket;

impl From<&S3Bucket> for BucketHttpModel {
    fn from(bucket: &S3Bucket) -> Self {
        Self {
            name: bucket.name.clone(),
        }
    }
}
