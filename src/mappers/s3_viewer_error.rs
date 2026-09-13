use crate::models::S3ViewerError;

// The error text an MCP tool call returns. Unlike the REST API (see http_server/errors.rs), the
// caller here is an agent that has to decide what to do next, so the S3 reason is passed through.
impl From<S3ViewerError> for String {
    fn from(err: S3ViewerError) -> Self {
        match err {
            S3ViewerError::BucketIsNotConfigured => {
                "Bucket is not configured. Call list_buckets to see the configured buckets"
                    .to_string()
            }
            S3ViewerError::S3(err) => format!("S3 request failed: {err}"),
        }
    }
}
