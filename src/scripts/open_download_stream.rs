use my_s3::S3DownloadStream;

use crate::models::S3ViewerError;
use crate::s3_buckets::S3Buckets;

// Opens the object for streaming. Awaited BEFORE the response starts, so "no such bucket" /
// "no such key" still become a proper 404 instead of an empty 200.
pub async fn open_download_stream(
    s3_buckets: &S3Buckets,
    bucket_name: &str,
    key: &str,
) -> Result<S3DownloadStream, S3ViewerError> {
    let bucket = s3_buckets
        .get(bucket_name)
        .ok_or(S3ViewerError::BucketIsNotConfigured)?;

    let stream = bucket
        .client
        .download_file_as_stream(bucket.name.as_str(), key)
        .await?;

    Ok(stream)
}
