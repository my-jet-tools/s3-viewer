use dioxus::prelude::*;

#[component]
pub fn BucketIcon() -> Element {
    rsx! {
        svg {
            class: "icon",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.9",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            ellipse { cx: "12", cy: "6.5", rx: "8", ry: "2.5" }
            path { d: "M4 6.5l1.9 11.6c.2 1.3 2.9 2.4 6.1 2.4s5.9-1.1 6.1-2.4L20 6.5" }
        }
    }
}
