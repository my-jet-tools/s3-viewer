use crate::models::Crumb;
use crate::states::{LevelKey, Selection};

/// `bucket / folder / ... / file` for the current selection. Every level segment points at its
/// level; the file segment of a selected file points nowhere.
pub fn build_crumbs(selection: &Selection) -> Vec<Crumb> {
    match selection {
        Selection::Level(key) => level_crumbs(key),
        Selection::File { bucket, key } => {
            let mut crumbs = level_crumbs(&LevelKey::parent_of(bucket, key));
            crumbs.push(Crumb {
                label: crate::utils::file_name_of(key).to_string(),
                target: None,
            });
            crumbs
        }
    }
}

fn level_crumbs(key: &LevelKey) -> Vec<Crumb> {
    key.path_from_root()
        .into_iter()
        .map(|level| {
            let label = level.name().to_string();
            Crumb {
                label,
                target: Some(level),
            }
        })
        .collect()
}
