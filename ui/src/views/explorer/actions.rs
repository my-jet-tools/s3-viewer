use dioxus::prelude::*;

use crate::states::{ExplorerState, LevelKey, Selection};

// Every async load goes through `spawn_forever`, not `spawn`: a load must outlive the component
// the click came from. A cancelled task would leave its level in `Loading` for good, and the
// loading guard in `ExplorerState` never fetches a `Loading` level again.

/// Loads the bucket list (also the Retry of a failed load). Safe to call from render: the state
/// write happens inside the spawned task, and the guard makes a second call a no-op.
pub fn load_buckets(mut cs: Signal<ExplorerState>) {
    dioxus::core::spawn_forever(async move {
        if !cs.write().begin_buckets_load() {
            return;
        }

        let result = crate::api::get_buckets().await;
        cs.write().set_buckets(result);
    });
}

/// Click on a tree row. `is_selected` is the row's selection as it was rendered: the first click of
/// every double click selects again, so an already selected row skips the write (and the re-render
/// of every pane) without opening the signal a second time to check.
pub fn select(mut cs: Signal<ExplorerState>, is_selected: bool, selection: Selection) {
    if is_selected {
        return;
    }

    cs.write().select(selection);
}

/// Double click on a bucket or folder in the tree.
pub fn activate_level(mut cs: Signal<ExplorerState>, key: LevelKey) {
    let must_fetch = cs.write().activate_level(&key);

    if must_fetch {
        fetch_level(cs, key);
    }
}

/// Click on the chevron of a bucket or folder.
pub fn toggle_level(mut cs: Signal<ExplorerState>, key: LevelKey) {
    let must_fetch = cs.write().toggle_level(&key);

    if must_fetch {
        fetch_level(cs, key);
    }
}

/// Select a level and open it in the tree together with every level above it
/// (double click on a folder in the right pane, the Open button, a breadcrumb segment).
pub fn reveal_level(mut cs: Signal<ExplorerState>, key: LevelKey) {
    let to_fetch = cs.write().reveal_level(&key);

    for level in to_fetch {
        fetch_level(cs, level);
    }
}

/// Refresh / Retry of one level, together with the loaded levels below it.
pub fn reload_level(mut cs: Signal<ExplorerState>, key: LevelKey) {
    let to_fetch = cs.write().restart_level_load(&key);

    for level in to_fetch {
        fetch_level(cs, level);
    }
}

/// Double click on a file in the tree: select it and download it. `is_selected` as in `select`.
pub fn open_file(cs: Signal<ExplorerState>, is_selected: bool, bucket: String, key: String) {
    download_file(&bucket, &key);
    select(cs, is_selected, Selection::File { bucket, key });
}

/// Starts the download without navigating the page (see `utils::start_download`), so an error
/// answer from the server can not replace the explorer.
pub fn download_file(bucket: &str, key: &str) {
    match crate::api::download_url(bucket, key) {
        Ok(url) => crate::utils::start_download(&url, crate::utils::file_name_of(key)),
        Err(err) => dioxus_utils::console_log(format!(
            "Can not build the download url of {bucket}/{key}: {}",
            err.message
        )),
    }
}

fn fetch_level(mut cs: Signal<ExplorerState>, key: LevelKey) {
    dioxus::core::spawn_forever(async move {
        let result = crate::api::list_objects(&key.bucket, &key.prefix).await;
        cs.write().set_level(key, result);
    });
}
