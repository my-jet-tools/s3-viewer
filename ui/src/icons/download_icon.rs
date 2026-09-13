use dioxus::prelude::*;

#[component]
pub fn DownloadIcon() -> Element {
    rsx! {
        svg {
            class: "icon",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M12 4v11" }
            path { d: "M7 10.5l5 5 5-5" }
            path { d: "M5 20h14" }
        }
    }
}
