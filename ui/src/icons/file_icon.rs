use dioxus::prelude::*;

#[component]
pub fn FileIcon() -> Element {
    rsx! {
        svg {
            class: "icon",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.9",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M14 3H7a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2V8z" }
            path { d: "M14 3v5h5" }
        }
    }
}
