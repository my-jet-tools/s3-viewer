use super::SettingsModel;

pub const SETTINGS_PATH_ENV_VARIABLE: &str = "S3_VIEWER_SETTINGS";

pub const DEFAULT_SETTINGS_PATH: &str = "~/.s3-viewer";

// Reads and validates the settings once at start-up. Every problem is a panic with a message that
// says what to fix: the viewer can not do anything useful without its bucket list.
pub async fn read_settings() -> SettingsModel {
    let file_name = get_settings_file_name();

    let content = match tokio::fs::read(file_name.as_str()).await {
        Ok(content) => content,
        Err(err) => panic!(
            "Can not read the settings file '{file_name}': {err}. Copy settings.example.yaml there, or point the {SETTINGS_PATH_ENV_VARIABLE} env variable at another file"
        ),
    };

    let settings: SettingsModel = match serde_yaml::from_slice(content.as_slice()) {
        Ok(settings) => settings,
        Err(err) => panic!(
            "The settings file '{file_name}' is not valid s3-viewer settings yaml (see settings.example.yaml): {err}"
        ),
    };

    if let Err(err) = super::validate_settings(&settings) {
        panic!("Invalid settings file '{file_name}': {err}");
    }

    settings
}

fn get_settings_file_name() -> String {
    let home = std::env::var("HOME").ok();

    if let Ok(path) = std::env::var(SETTINGS_PATH_ENV_VARIABLE) {
        let path = path.trim();

        if !path.is_empty() {
            return super::expand_home_dir(path, home.as_deref());
        }
    }

    super::expand_home_dir(DEFAULT_SETTINGS_PATH, home.as_deref())
}
