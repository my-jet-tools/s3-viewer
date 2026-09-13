use dioxus::prelude::*;

/// Points right; the expanded state rotates it by CSS.
#[component]
pub fn ChevronIcon() -> Element {
    rsx! {
        svg {
            class: "icon",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2.4",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M9 6l6 6-6 6" }
        }
    }
}
