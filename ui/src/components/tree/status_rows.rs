use dioxus::prelude::*;

use crate::components::Spinner;
use crate::icons::WarningIcon;
use crate::states::{ExplorerState, LevelKey};

// The non-item rows a level can show under its node. They share the row geometry of the items,
// so their text lines up with the names of the level's children.

pub fn render_loading_row(depth: usize) -> Element {
    let indent = super::row_indent(depth);

    rsx! {
        div { class: "tree-row tree-row--status", style: "padding-left: {indent}px",
            span { class: "tree-row__spacer" }
            Spinner {}
            span { class: "tree-row__label", "Loading…" }
        }
    }
}

pub fn render_error_row(
    cs: Signal<ExplorerState>,
    key: &LevelKey,
    message: &str,
    depth: usize,
) -> Element {
    let indent = super::row_indent(depth);
    let retry_key = key.clone();

    rsx! {
        div {
            class: "tree-row tree-row--status tree-row--error",
            style: "padding-left: {indent}px",
            title: "{message}",
            span { class: "tree-row__spacer" }
            span { class: "tree-row__label", "Error: {message}" }
            button {
                class: "tree-row__retry",
                onclick: move |evt| {
                    evt.stop_propagation();
                    crate::views::reload_level(cs, retry_key.clone());
                },
                "Retry"
            }
        }
    }
}

pub fn render_muted_row(depth: usize, text: &str) -> Element {
    let indent = super::row_indent(depth);

    rsx! {
        div {
            class: "tree-row tree-row--status tree-row--muted",
            style: "padding-left: {indent}px",
            span { class: "tree-row__spacer" }
            span { class: "tree-row__label", "{text}" }
        }
    }
}

pub fn render_warning_row(depth: usize, text: &str) -> Element {
    let indent = super::row_indent(depth);

    rsx! {
        div {
            class: "tree-row tree-row--status tree-row--warning",
            style: "padding-left: {indent}px",
            title: "The server stopped paging before the end of this folder",
            span { class: "tree-row__spacer" }
            WarningIcon {}
            span { class: "tree-row__label", "{text}" }
        }
    }
}
