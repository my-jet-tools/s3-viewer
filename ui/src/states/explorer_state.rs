use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use dioxus_utils::{DataState, RenderState};
use rest_api_shared::{BucketHttpModel, FileHttpModel, ListObjectsResponse};

use super::{LevelKey, LevelStatus, Selection};
use crate::models::RequestError;

/// The explorer state, provided ONCE as context (`Signal<ExplorerState>`) by the explorer page.
///
/// One shared state rather than a `DataState` per component, deliberately: the tree and the right
/// pane show the SAME folder listing. Each level is fetched once, cached here under its
/// `LevelKey`, and both panes render the same `Rc` - component-local states would fetch every
/// folder twice and could show two different listings after a Refresh.
///
/// Every operation is one method, so a handler makes exactly one write. The methods that may
/// start a load say whether (or which levels) the caller has to fetch; the loading guard lives
/// here: a level that is `Loading` or `Loaded` is never fetched again by open / toggle, however
/// fast the double clicks come.
#[derive(Default)]
pub struct ExplorerState {
    pub buckets: DataState<Vec<BucketHttpModel>>,
    pub levels: HashMap<LevelKey, DataState<Rc<ListObjectsResponse>>>,
    pub expanded: HashSet<LevelKey>,
    pub selected: Option<Selection>,
}

impl ExplorerState {
    /// `true` when the caller has to fetch the buckets: nothing loaded yet, or the last load failed.
    pub fn begin_buckets_load(&mut self) -> bool {
        match self.buckets.as_ref() {
            RenderState::None | RenderState::Error(_) => {
                self.buckets.set_loading();
                true
            }
            RenderState::Loading | RenderState::Loaded(_) => false,
        }
    }

    pub fn set_buckets(&mut self, result: Result<Vec<BucketHttpModel>, RequestError>) {
        match result {
            Ok(buckets) => self.buckets.set_value(buckets),
            // DataState keeps `format!("{:?}", err)`; format_args! keeps the message unquoted.
            Err(err) => self.buckets.set_error(format_args!("{}", err.message)),
        }
    }

    pub fn select(&mut self, selection: Selection) {
        self.selected = Some(selection);
    }

    /// Double click on a bucket or folder: select it; a level that is not loaded starts loading
    /// and expands, a loaded one toggles. `true` when the caller has to fetch the level.
    pub fn activate_level(&mut self, key: &LevelKey) -> bool {
        self.selected = Some(Selection::Level(key.clone()));

        match self.level_status(key) {
            LevelStatus::NotLoaded => {
                self.start_level_load(key);
                self.expanded.insert(key.clone());
                true
            }
            LevelStatus::Loading => {
                self.expanded.insert(key.clone());
                false
            }
            LevelStatus::Loaded => {
                self.toggle_expanded(key);
                false
            }
        }
    }

    /// Click on the chevron: collapse an expanded level, expand a collapsed one - loading it when
    /// it is not loaded yet. `true` when the caller has to fetch the level.
    pub fn toggle_level(&mut self, key: &LevelKey) -> bool {
        if self.expanded.remove(key) {
            return false;
        }

        self.expanded.insert(key.clone());

        match self.level_status(key) {
            LevelStatus::NotLoaded => {
                self.start_level_load(key);
                true
            }
            LevelStatus::Loading | LevelStatus::Loaded => false,
        }
    }

    /// Select a level and make it visible in the tree: every level from the bucket root down to it
    /// is expanded, and the ones not loaded yet start loading. Returns the levels to fetch.
    pub fn reveal_level(&mut self, key: &LevelKey) -> Vec<LevelKey> {
        self.selected = Some(Selection::Level(key.clone()));

        let mut to_fetch = Vec::new();

        for level in key.path_from_root() {
            match self.level_status(&level) {
                LevelStatus::NotLoaded => {
                    self.start_level_load(&level);
                    to_fetch.push(level.clone());
                }
                LevelStatus::Loading | LevelStatus::Loaded => {}
            }

            self.expanded.insert(level);
        }

        to_fetch
    }

    /// Refresh / Retry: drop the cached listing of the level and load it again - together with the
    /// cached listings BELOW it, or the open sub-folders would keep showing their old contents.
    /// The expanded state is left as it is: a level below that is expanded is loaded again as
    /// well, a collapsed one only loses its cache and loads when it is opened. A level that is
    /// loading already (this one or one below) is left to its running task - when it is this
    /// level, nothing is restarted. Returns the levels the caller has to fetch.
    pub fn restart_level_load(&mut self, key: &LevelKey) -> Vec<LevelKey> {
        let mut to_fetch = Vec::new();

        match self.level_status(key) {
            LevelStatus::Loading => return to_fetch,
            LevelStatus::NotLoaded | LevelStatus::Loaded => {
                self.start_level_load(key);
                to_fetch.push(key.clone());
            }
        }

        let below: Vec<LevelKey> = self
            .levels
            .keys()
            .filter(|level| level.is_below(key))
            .cloned()
            .collect();

        for level in below {
            match self.level_status(&level) {
                LevelStatus::Loading => {}
                LevelStatus::NotLoaded | LevelStatus::Loaded => {
                    if self.expanded.contains(&level) {
                        self.start_level_load(&level);
                        to_fetch.push(level);
                    } else {
                        self.levels.remove(&level);
                    }
                }
            }
        }

        to_fetch
    }

    pub fn set_level(&mut self, key: LevelKey, result: Result<ListObjectsResponse, RequestError>) {
        let level = self.levels.entry(key).or_default();

        match result {
            Ok(listing) => level.set_value(Rc::new(listing)),
            Err(err) => level.set_error(format_args!("{}", err.message)),
        }
    }

    pub fn level_render_state(&self, key: &LevelKey) -> Option<&RenderState<Rc<ListObjectsResponse>>> {
        self.levels.get(key).map(|level| level.as_ref())
    }

    pub fn is_expanded(&self, key: &LevelKey) -> bool {
        self.expanded.contains(key)
    }

    pub fn is_level_selected(&self, key: &LevelKey) -> bool {
        match &self.selected {
            Some(Selection::Level(selected)) => selected == key,
            Some(Selection::File { .. }) | None => false,
        }
    }

    pub fn is_file_selected(&self, bucket: &str, key: &str) -> bool {
        match &self.selected {
            Some(Selection::File {
                bucket: selected_bucket,
                key: selected_key,
            }) => selected_bucket == bucket && selected_key == key,
            Some(Selection::Level(_)) | None => false,
        }
    }

    /// A file's size and date live in the listing of its parent level.
    pub fn find_file(&self, bucket: &str, key: &str) -> Option<&FileHttpModel> {
        let parent = LevelKey::parent_of(bucket, key);
        let listing = self.levels.get(&parent)?.try_unwrap_as_loaded()?;

        listing.files.iter().find(|file| file.key == key)
    }

    fn level_status(&self, key: &LevelKey) -> LevelStatus {
        match self.level_render_state(key) {
            None | Some(RenderState::None) | Some(RenderState::Error(_)) => LevelStatus::NotLoaded,
            Some(RenderState::Loading) => LevelStatus::Loading,
            Some(RenderState::Loaded(_)) => LevelStatus::Loaded,
        }
    }

    fn start_level_load(&mut self, key: &LevelKey) {
        self.levels.entry(key.clone()).or_default().set_loading();
    }

    fn toggle_expanded(&mut self, key: &LevelKey) {
        if !self.expanded.remove(key) {
            self.expanded.insert(key.clone());
        }
    }
}
