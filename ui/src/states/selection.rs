use super::LevelKey;

/// What the right pane and the breadcrumb show.
#[derive(Clone, Debug, PartialEq)]
pub enum Selection {
    Level(LevelKey),
    File { bucket: String, key: String },
}
