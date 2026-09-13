use dioxus::prelude::MouseEvent;
use dioxus::web::WebEventExt;

/// `true` for the second (third, ...) click of a multi-click. A double click fires `click` twice
/// before `dblclick`, so a button that acts on click has to skip the repeat to act once.
pub fn is_repeated_click(evt: &MouseEvent) -> bool {
    match evt.data().try_as_web_event() {
        Some(web_event) => web_event.detail() > 1,
        None => false,
    }
}
