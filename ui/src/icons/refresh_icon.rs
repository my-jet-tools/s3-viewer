use dioxus::prelude::*;

#[component]
pub fn RefreshIcon() -> Element {
    rsx! {
        svg {
            class: "icon",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M20.5 12a8.5 8.5 0 1 1-2.5-6" }
            path { d: "M20.5 3.5V9H15" }
        }
    }
}
