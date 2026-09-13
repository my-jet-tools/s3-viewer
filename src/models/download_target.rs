// What a running download is about - the log context when the S3 stream breaks mid-way.
// Credentials are deliberately not part of it.
pub struct DownloadTarget {
    pub bucket: String,
    pub key: String,
}
