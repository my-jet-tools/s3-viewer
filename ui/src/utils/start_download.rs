use web_sys::wasm_bindgen::JsCast;

/// Starts the download of `url` WITHOUT navigating the page: a detached `<a href download>` is
/// clicked. The server answers with `Content-Disposition: attachment`; when it answers with an
/// error instead (a deleted key, a bucket not in the settings, S3 failing), the browser reports a
/// failed download and the explorer - tree, expanded levels, loaded listings - stays as it is.
/// A top-level navigation would have replaced the whole app with the error body.
///
/// `file_name` is only the fallback name: the server's Content-Disposition name wins.
pub fn start_download(url: &str, file_name: &str) {
    let Some(document) = web_sys::window().and_then(|window| window.document()) else {
        dioxus_utils::console_log("Can not start a download: there is no document");
        return;
    };

    let Some(body) = document.body() else {
        dioxus_utils::console_log("Can not start a download: the document has no body");
        return;
    };

    let anchor = match document.create_element("a") {
        Ok(element) => element,
        Err(err) => {
            dioxus_utils::console_log(format!("Can not start a download of {url}: {err:?}"));
            return;
        }
    };

    let Ok(anchor) = anchor.dyn_into::<web_sys::HtmlAnchorElement>() else {
        dioxus_utils::console_log(format!("Can not start a download of {url}: <a> is not an anchor"));
        return;
    };

    anchor.set_href(url);
    anchor.set_download(file_name);

    // Firefox only follows a click on an anchor that is attached to the document.
    if let Err(err) = body.append_child(&anchor) {
        dioxus_utils::console_log(format!("Can not start a download of {url}: {err:?}"));
        return;
    }

    anchor.click();
    anchor.remove();
}
