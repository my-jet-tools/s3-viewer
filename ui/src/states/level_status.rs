/// What the cached listing of a level means for the next open / toggle / refresh.
/// A failed load counts as `NotLoaded`: opening the level again is the retry.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LevelStatus {
    NotLoaded,
    Loading,
    Loaded,
}
