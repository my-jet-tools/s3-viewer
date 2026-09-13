use dioxus::prelude::*;

use crate::states::{ExplorerState, LevelKey};

/// A selected bucket or folder whose level has not been loaded yet.
pub fn render_not_opened_hint(cs: Signal<ExplorerState>, key: &LevelKey) -> Element {
    let what = if key.is_bucket_root() { "bucket" } else { "folder" };
    let open_key = key.clone();

    rsx! {
        div { class: "placeholder",
            div { class: "placeholder__title", "Double-click to open" }
            div { class: "placeholder__text", "The contents of this {what} are not loaded yet." }
            button {
                class: "btn btn--primary",
                onclick: move |_| crate::views::reveal_level(cs, open_key.clone()),
                "Open"
            }
        }
    }
}
