use dioxus::prelude::*;

use super::SplitterState;
use crate::components::{DetailsPane, TopBar, TreePane};
use crate::states::ExplorerState;

#[component]
pub fn ExplorerPage() -> Element {
    // One Signal<ExplorerState> for the whole page, shared by context: the tree, the right pane and
    // the breadcrumb all show the same cached listings (see ExplorerState for why it is not
    // component-local).
    use_context_provider(|| Signal::new(ExplorerState::default()));

    let mut cs = use_signal(SplitterState::default);
    let cs_ra = cs.read();

    let tree_width = cs_ra.tree_width;
    // The handlers below capture `dragging` as this render saw it (start_drag re-renders the page,
    // so they see `true` during a drag). The pointer moves over the workspace all the time, and
    // only a drag may write: one signal access per event, and none at all when nothing is dragged.
    let dragging = cs_ra.dragging;
    let workspace_class = if dragging {
        "workspace is-resizing"
    } else {
        "workspace"
    };

    rsx! {
        div { class: "app",
            TopBar {}
            div {
                class: workspace_class,
                onmousemove: move |evt| {
                    if dragging {
                        cs.write().drag_to(evt.client_coordinates().x);
                    }
                },
                onmouseup: move |_| {
                    if dragging {
                        cs.write().stop_drag();
                    }
                },
                onmouseleave: move |_| {
                    if dragging {
                        cs.write().stop_drag();
                    }
                },
                div { class: "tree-pane", style: "width: {tree_width}px", TreePane {} }
                div {
                    class: "splitter",
                    title: "Drag to resize",
                    onmousedown: move |evt| {
                        // no text selection while dragging
                        evt.prevent_default();
                        cs.write().start_drag();
                    },
                }
                div { class: "details-pane", DetailsPane {} }
            }
        }
    }
}
