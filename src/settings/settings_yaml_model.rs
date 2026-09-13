use serde::Deserialize;

// The settings file as written - see settings.example.yaml. `validate_settings` turns it into
// `SettingsModel`, parsing every bucket connection string on the way.
#[derive(Deserialize)]
pub struct SettingsYamlModel {
    pub http_port: Option<u16>,
    pub buckets: Vec<String>,
}
