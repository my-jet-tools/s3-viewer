use super::S3ConnString;

pub const DEFAULT_HTTP_PORT: u16 = 8000;

// Validated settings: the port resolved and every bucket connection string parsed, in settings
// order. Built by `validate_settings` from the file (`SettingsYamlModel`).
pub struct SettingsModel {
    pub http_port: u16,
    pub buckets: Vec<S3ConnString>,
}

impl SettingsModel {
    pub fn get_http_port(&self) -> u16 {
        self.http_port
    }
}
