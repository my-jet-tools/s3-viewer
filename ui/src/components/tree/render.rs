use dioxus::prelude::*;
use dioxus_utils::RenderState;
use rest_api_shared::BucketHttpModel;

use super::NodeKind;
use crate::states::{ExplorerState, LevelKey};

/// The left pane. It reads the shared state once and renders the whole tree through plain
/// functions, so one component subscribes to the state instead of one per node.
#[component]
pub fn TreePane() -> Element {
    let cs = use_context::<Signal<ExplorerState>>();
    let cs_ra = cs.read();

    let bucket_count = cs_ra.buckets.try_unwrap_as_loaded().map(|buckets| buckets.len());

    // The header stays while the buckets load or fail: the loading / error element goes into the body.
    let body = match get_buckets(cs, &cs_ra) {
        Ok(buckets) => render_buckets(cs, &cs_ra, buckets),
        Err(el) => el,
    };

    rsx! {
        div { class: "tree",
            div { class: "tree__header",
                span { "Buckets" }
                if let Some(count) = bucket_count {
                    span { class: "tree__count", "{count}" }
                }
            }
            div { class: "tree__body", {body} }
        }
    }
}

fn get_buckets(
    cs: Signal<ExplorerState>,
    cs_ra: &ExplorerState,
) -> Result<&[BucketHttpModel], Element> {
    match cs_ra.buckets.as_ref() {
        RenderState::None => {
            // Only spawns the load: the state write happens inside the task, not during render.
            crate::views::load_buckets(cs);
            Err(super::render_loading_row(0))
        }
        RenderState::Loading => Err(super::render_loading_row(0)),
        RenderState::Error(message) => Err(rsx! {
            div { class: "tree__message",
                crate::components::ErrorBox {
                    message: message.clone(),
                    on_retry: move |_| crate::views::load_buckets(cs),
                }
            }
        }),
        RenderState::Loaded(buckets) => Ok(buckets.as_slice()),
    }
}

fn render_buckets(
    cs: Signal<ExplorerState>,
    state: &ExplorerState,
    buckets: &[BucketHttpModel],
) -> Element {
    if buckets.is_empty() {
        return super::render_muted_row(0, "No buckets configured");
    }

    let nodes = buckets.iter().map(|bucket| {
        super::render_node(
            cs,
            state,
            LevelKey::bucket_root(&bucket.name),
            &bucket.name,
            NodeKind::Bucket,
            0,
        )
    });

    rsx! {
        {nodes}
    }
}
