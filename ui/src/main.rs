use dioxus::prelude::*;

mod api;
mod components;
mod icons;
mod models;
mod states;
mod utils;
mod views;

#[derive(Routable, PartialEq, Clone)]
pub enum AppRoute {
    #[route("/")]
    Explorer {},

    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}

fn main() {
    dioxus::LaunchBuilder::new().launch(|| {
        rsx! {
            // favicon.ico for the browsers without SVG favicons; `sizes` keeps the browsers that
            // understand both on the SVG.
            document::Link {
                rel: "icon",
                sizes: "32x32",
                href: asset!("/public/favicon.ico"),
            }
            document::Link {
                rel: "icon",
                r#type: "image/svg+xml",
                href: asset!("/public/favicon.svg"),
            }
            Router::<AppRoute> {}
        }
    });
}

#[component]
fn Explorer() -> Element {
    rsx! {
        crate::views::ExplorerPage {}
    }
}

#[component]
fn NotFound(segments: Vec<String>) -> Element {
    let path = format!("/{}", segments.join("/"));

    rsx! {
        div { class: "not-found",
            div { class: "not-found__card",
                div { class: "not-found__title", "Page not found" }
                div { class: "muted",
                    "Nothing lives at "
                    span { class: "mono", "{path}" }
                }
                Link { class: "btn btn--primary", to: AppRoute::Explorer {}, "Open the explorer" }
            }
        }
    }
}
