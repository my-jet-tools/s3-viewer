use dioxus::prelude::*;
use rest_api_shared::FileHttpModel;

use crate::icons::FileIcon;
use crate::states::{ExplorerState, Selection};

/// A file row: a click selects it, a double click downloads it.
pub fn render_file_row(
    cs: Signal<ExplorerState>,
    state: &ExplorerState,
    bucket: &str,
    file: &FileHttpModel,
    depth: usize,
) -> Element {
    let is_selected = state.is_file_selected(bucket, &file.key);
    let row_class = if is_selected {
        "tree-row is-selected"
    } else {
        "tree-row"
    };
    let indent = super::row_indent(depth);

    let selection = Selection::File {
        bucket: bucket.to_string(),
        key: file.key.clone(),
    };
    let open_bucket = bucket.to_string();
    let open_key = file.key.clone();

    rsx! {
        div {
            key: "{file.key}",
            class: row_class,
            style: "padding-left: {indent}px",
            title: "{file.name}",
            onclick: move |_| crate::views::select(cs, is_selected, selection.clone()),
            ondoubleclick: move |_| {
                crate::views::open_file(cs, is_selected, open_bucket.clone(), open_key.clone())
            },
            span { class: "tree-row__spacer" }
            span { class: "tree-row__icon is-file", FileIcon {} }
            span { class: "tree-row__label", "{file.name}" }
        }
    }
}
