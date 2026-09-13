use dioxus::prelude::*;

/// A failed request inside a pane: the server's message and a Retry button.
#[component]
pub fn ErrorBox(message: String, on_retry: EventHandler<()>) -> Element {
    rsx! {
        div { class: "error-box", role: "alert",
            div { class: "error-box__text",
                div { class: "error-box__title", "Request failed" }
                div { class: "error-box__message", "{message}" }
            }
            button { class: "btn btn--sm", onclick: move |_| on_retry.call(()), "Retry" }
        }
    }
}
