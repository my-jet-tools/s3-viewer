use crate::states::LevelKey;

/// One segment of the top-bar breadcrumb. `target` is the level a click on the segment selects;
/// the file segment of a selected file has none.
#[derive(Clone, Debug, PartialEq)]
pub struct Crumb {
    pub label: String,
    pub target: Option<LevelKey>,
}
