use dioxus::prelude::*;
use dioxus_utils::RenderState;
use rest_api_shared::ListObjectsResponse;

use super::NodeKind;
use crate::states::{ExplorerState, LevelKey};

/// The rows under an expanded bucket or folder: loading, error with Retry, "empty", or its
/// folders and files - plus a warning row when the server truncated the listing.
pub fn render_level_rows(
    cs: Signal<ExplorerState>,
    state: &ExplorerState,
    key: &LevelKey,
    depth: usize,
) -> Element {
    let listing = match get_level(cs, state, key, depth) {
        Ok(listing) => listing,
        Err(el) => return el,
    };

    render_listing(cs, state, key, listing, depth)
}

/// The listing of an expanded level, or the row standing in for it. A level is fetched by the
/// click that expanded it (or by Refresh), never from render - so no data yet reads as loading.
fn get_level<'a>(
    cs: Signal<ExplorerState>,
    state: &'a ExplorerState,
    key: &LevelKey,
    depth: usize,
) -> Result<&'a ListObjectsResponse, Element> {
    match state.level_render_state(key) {
        None | Some(RenderState::None) | Some(RenderState::Loading) => {
            Err(super::render_loading_row(depth))
        }
        Some(RenderState::Error(message)) => Err(super::render_error_row(cs, key, message, depth)),
        Some(RenderState::Loaded(listing)) => Ok(listing.as_ref()),
    }
}

fn render_listing(
    cs: Signal<ExplorerState>,
    state: &ExplorerState,
    key: &LevelKey,
    listing: &ListObjectsResponse,
    depth: usize,
) -> Element {
    if listing.folders.is_empty() && listing.files.is_empty() && !listing.is_truncated {
        return super::render_muted_row(depth, "empty");
    }

    let folders = listing.folders.iter().map(|folder| {
        super::render_node(
            cs,
            state,
            LevelKey::folder(&key.bucket, &folder.prefix),
            &folder.name,
            NodeKind::Folder,
            depth,
        )
    });

    let files = listing
        .files
        .iter()
        .map(|file| super::render_file_row(cs, state, &key.bucket, file, depth));

    rsx! {
        {folders}
        {files}
        if listing.is_truncated {
            {super::render_warning_row(depth, "Truncated: not every item is shown")}
        }
    }
}
