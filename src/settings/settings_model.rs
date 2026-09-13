use serde::Deserialize;

use super::BucketSettingsModel;

pub const DEFAULT_HTTP_PORT: u16 = 8000;

// The format is documented by settings.example.yaml at the repo root.
#[derive(Deserialize)]
pub struct SettingsModel {
    pub http_port: Option<u16>,
    pub buckets: Vec<BucketSettingsModel>,
}

impl SettingsModel {
    pub fn get_http_port(&self) -> u16 {
        self.http_port.unwrap_or(DEFAULT_HTTP_PORT)
    }
}
