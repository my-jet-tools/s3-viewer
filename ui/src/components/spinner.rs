use dioxus::prelude::*;

#[component]
pub fn Spinner() -> Element {
    rsx! {
        span { class: "spinner", aria_hidden: "true" }
    }
}
