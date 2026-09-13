use dioxus::prelude::*;

use crate::models::Crumb;
use crate::states::ExplorerState;

#[component]
pub fn TopBar() -> Element {
    let cs = use_context::<Signal<ExplorerState>>();
    let cs_ra = cs.read();

    let crumbs = match cs_ra.selected.as_ref() {
        Some(selection) => super::build_crumbs(selection),
        None => Vec::new(),
    };

    let nothing_selected = crumbs.is_empty();
    let last_index = crumbs.len().saturating_sub(1);
    let crumb_nodes = crumbs
        .into_iter()
        .enumerate()
        .map(move |(index, crumb)| render_crumb(cs, index, crumb, index == last_index));

    rsx! {
        header { class: "topbar",
            div { class: "topbar__brand",
                img {
                    class: "topbar__logo",
                    src: asset!("/public/favicon.svg"),
                    alt: "",
                }
                span { class: "topbar__title", "S3 Viewer" }
            }
            nav { class: "breadcrumb",
                if nothing_selected {
                    span { class: "breadcrumb__placeholder", "Nothing selected" }
                }
                {crumb_nodes}
            }
        }
    }
}

fn render_crumb(cs: Signal<ExplorerState>, index: usize, crumb: Crumb, is_last: bool) -> Element {
    let label = crumb.label;

    let content = match crumb.target {
        Some(target) if !is_last => rsx! {
            button {
                class: "breadcrumb__link",
                title: "{label}",
                onclick: move |_| crate::views::reveal_level(cs, target.clone()),
                "{label}"
            }
        },
        Some(_) | None => rsx! {
            span { class: "breadcrumb__current", title: "{label}", "{label}" }
        },
    };

    rsx! {
        span { key: "{index}", class: "breadcrumb__item",
            if index > 0 {
                span { class: "breadcrumb__sep", "/" }
            }
            {content}
        }
    }
}
