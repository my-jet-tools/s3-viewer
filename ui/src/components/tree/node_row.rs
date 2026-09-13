use dioxus::prelude::*;

use super::NodeKind;
use crate::icons::{BucketIcon, ChevronIcon, FolderIcon};
use crate::states::{ExplorerState, LevelKey, Selection};

/// A bucket or folder row, followed by the rows of its level when it is expanded.
pub fn render_node(
    cs: Signal<ExplorerState>,
    state: &ExplorerState,
    key: LevelKey,
    name: &str,
    kind: NodeKind,
    depth: usize,
) -> Element {
    let expanded = state.is_expanded(&key);
    let is_selected = state.is_level_selected(&key);
    let row_class = if is_selected {
        "tree-row is-selected"
    } else {
        "tree-row"
    };
    let chevron_class = if expanded {
        "tree-row__chevron is-expanded"
    } else {
        "tree-row__chevron"
    };
    let indent = super::row_indent(depth);

    let icon = match kind {
        NodeKind::Bucket => rsx! {
            span { class: "tree-row__icon is-bucket", BucketIcon {} }
        },
        NodeKind::Folder => rsx! {
            span { class: "tree-row__icon is-folder", FolderIcon {} }
        },
    };

    let children = if expanded {
        super::render_level_rows(cs, state, &key, depth + 1)
    } else {
        rsx! {}
    };

    let dom_key = format!("{}/{}", key.bucket, key.prefix);
    let select_key = key.clone();
    let activate_key = key.clone();
    let toggle_key = key;

    rsx! {
        div { key: "{dom_key}", class: "tree-node",
            div {
                class: row_class,
                style: "padding-left: {indent}px",
                title: "{name}",
                onclick: move |_| {
                    crate::views::select(cs, is_selected, Selection::Level(select_key.clone()))
                },
                ondoubleclick: move |_| crate::views::activate_level(cs, activate_key.clone()),
                span {
                    class: chevron_class,
                    onclick: move |evt| {
                        evt.stop_propagation();
                        crate::views::toggle_level(cs, toggle_key.clone());
                    },
                    // A double click on the chevron is already two toggles - it must not open the row too.
                    ondoubleclick: move |evt| evt.stop_propagation(),
                    ChevronIcon {}
                }
                {icon}
                span { class: "tree-row__label", "{name}" }
            }
            {children}
        }
    }
}
