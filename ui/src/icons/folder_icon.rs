use dioxus::prelude::*;

#[component]
pub fn FolderIcon() -> Element {
    rsx! {
        svg { class: "icon", view_box: "0 0 24 24", fill: "currentColor",
            path { d: "M3 6.5A2.5 2.5 0 0 1 5.5 4h3.6c.7 0 1.3.3 1.8.8l1.4 1.5h6.2A2.5 2.5 0 0 1 21 8.8v8.7a2.5 2.5 0 0 1-2.5 2.5h-13A2.5 2.5 0 0 1 3 17.5z" }
        }
    }
}
