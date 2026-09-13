use dioxus::prelude::*;

pub fn render_nothing_selected() -> Element {
    rsx! {
        div { class: "placeholder",
            div { class: "placeholder__title", "Nothing selected" }
            div { class: "placeholder__text",
                "Pick a bucket in the tree. Double-click a bucket or folder to open it, double-click a file to download it."
            }
        }
    }
}
