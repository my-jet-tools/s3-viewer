use serde::Deserialize;

// One root node of the tree. `name` is the S3 bucket name and the key the UI addresses it by.
// No `Debug` on purpose: the struct carries credentials and must never end up in a log line.
#[derive(Deserialize)]
pub struct BucketSettingsModel {
    pub name: String,
    pub endpoint: String,
    pub region: String,
    pub access_key: String,
    pub secret_key: String,
}
