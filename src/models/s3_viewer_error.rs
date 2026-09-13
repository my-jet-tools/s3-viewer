use my_s3::S3Error;

// Why a bucket operation failed. Mapped to an HTTP answer in `http_server/errors.rs`.
pub enum S3ViewerError {
    // The bucket name from the request is not in the settings file.
    BucketIsNotConfigured,
    S3(S3Error),
}

impl From<S3Error> for S3ViewerError {
    fn from(err: S3Error) -> Self {
        Self::S3(err)
    }
}
