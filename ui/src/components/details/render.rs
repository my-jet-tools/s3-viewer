use dioxus::prelude::*;

use crate::states::{ExplorerState, Selection};

/// The right pane: what is known about the current selection.
#[component]
pub fn DetailsPane() -> Element {
    let cs = use_context::<Signal<ExplorerState>>();
    let cs_ra = cs.read();

    let content = match cs_ra.selected.as_ref() {
        None => super::render_nothing_selected(),
        Some(Selection::Level(key)) => super::render_level_details(cs, &cs_ra, key),
        Some(Selection::File { bucket, key }) => super::render_file_card(&cs_ra, bucket, key),
    };

    rsx! {
        div { class: "details", {content} }
    }
}
