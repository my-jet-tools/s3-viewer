use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_utils::RenderState;
use rest_api_shared::ListObjectsResponse;

use crate::components::{ErrorBox, Spinner};
use crate::icons::{BucketIcon, FolderIcon, RefreshIcon};
use crate::states::{ExplorerState, LevelKey};

/// A selected bucket or folder: its path with a Refresh button, then its listing - or the hint
/// to open it, the loading indicator or the error.
pub fn render_level_details(
    cs: Signal<ExplorerState>,
    state: &ExplorerState,
    key: &LevelKey,
) -> Element {
    let status = state.level_render_state(key);
    let header = render_header(cs, key, status);

    // The header stays in every state: the hint / loading / error element goes below it.
    let body = match get_level(cs, state, key) {
        Ok(listing) => super::render_listing_table(cs, key, listing),
        Err(el) => el,
    };

    rsx! {
        {header}
        {body}
    }
}

/// The listing of the selected level, or what stands in for it. A level that was never opened is
/// not fetched from here: selecting is not opening, so it gets the hint with an Open button.
fn get_level<'a>(
    cs: Signal<ExplorerState>,
    state: &'a ExplorerState,
    key: &LevelKey,
) -> Result<&'a ListObjectsResponse, Element> {
    match state.level_render_state(key) {
        None | Some(RenderState::None) => Err(super::render_not_opened_hint(cs, key)),
        Some(RenderState::Loading) => Err(rsx! {
            div { class: "pane-loading",
                Spinner {}
                span { "Loading…" }
            }
        }),
        Some(RenderState::Error(message)) => {
            let retry_key = key.clone();
            Err(rsx! {
                ErrorBox {
                    message: message.clone(),
                    on_retry: move |_| crate::views::reload_level(cs, retry_key.clone()),
                }
            })
        }
        Some(RenderState::Loaded(listing)) => Ok(listing.as_ref()),
    }
}

fn render_header(
    cs: Signal<ExplorerState>,
    key: &LevelKey,
    status: Option<&RenderState<Rc<ListObjectsResponse>>>,
) -> Element {
    let title = key.name();
    let path = format!("s3://{}/{}", key.bucket, key.prefix);

    let icon = if key.is_bucket_root() {
        rsx! {
            span { class: "details-header__icon is-bucket", BucketIcon {} }
        }
    } else {
        rsx! {
            span { class: "details-header__icon is-folder", FolderIcon {} }
        }
    };

    let meta = match status {
        Some(RenderState::Loaded(listing)) => Some(format!(
            "{} · {}",
            crate::utils::pluralize(listing.folders.len(), "folder", "folders"),
            crate::utils::pluralize(listing.files.len(), "file", "files"),
        )),
        None | Some(RenderState::None) | Some(RenderState::Loading) | Some(RenderState::Error(_)) => {
            None
        }
    };

    let action = match status {
        None | Some(RenderState::None) => rsx! {},
        Some(RenderState::Loading) => rsx! {
            button { class: "btn", disabled: true,
                Spinner {}
                "Refresh"
            }
        },
        Some(RenderState::Error(_)) | Some(RenderState::Loaded(_)) => {
            let refresh_key = key.clone();
            rsx! {
                button {
                    class: "btn",
                    title: "Reload this level from S3",
                    onclick: move |_| crate::views::reload_level(cs, refresh_key.clone()),
                    RefreshIcon {}
                    "Refresh"
                }
            }
        }
    };

    rsx! {
        div { class: "details-header",
            div { class: "details-header__main",
                {icon}
                div { class: "details-header__texts",
                    h2 { class: "details-header__title", title: "{title}", "{title}" }
                    div { class: "details-header__path mono", title: "{path}", "{path}" }
                    if let Some(meta) = meta {
                        div { class: "details-header__meta", "{meta}" }
                    }
                }
            }
            {action}
        }
    }
}
