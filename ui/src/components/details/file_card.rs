use dioxus::prelude::*;

use crate::icons::{DownloadIcon, FileIcon};
use crate::states::ExplorerState;

/// A selected file: name, full key, size, last modified and a Download button.
/// Size and date come from the cached listing of the file's folder.
pub fn render_file_card(state: &ExplorerState, bucket: &str, key: &str) -> Element {
    let file = state.find_file(bucket, key);

    let name = match file {
        Some(file) => file.name.as_str(),
        None => crate::utils::file_name_of(key),
    };

    let size = match file {
        Some(file) => format!(
            "{} ({} bytes)",
            crate::utils::format_size(file.size),
            crate::utils::format_bytes_exact(file.size)
        ),
        None => "—".to_string(),
    };

    let modified = match file {
        Some(file) => crate::utils::format_last_modified(&file.last_modified),
        None => "—".to_string(),
    };

    let is_known = file.is_some();
    let download_bucket = bucket.to_string();
    let download_key = key.to_string();

    rsx! {
        div { class: "file-card",
            div { class: "file-card__header",
                span { class: "details-header__icon is-file", FileIcon {} }
                div { class: "details-header__texts",
                    h2 { class: "details-header__title", title: "{name}", "{name}" }
                    div { class: "details-header__path mono", title: "s3://{bucket}/{key}",
                        "s3://{bucket}/{key}"
                    }
                }
            }
            div { class: "file-card__body",
                dl { class: "props",
                    dt { "Name" }
                    dd { "{name}" }
                    dt { "Bucket" }
                    dd { "{bucket}" }
                    dt { "Key" }
                    dd { class: "mono", "{key}" }
                    dt { "Size" }
                    dd { "{size}" }
                    dt { "Last modified" }
                    dd { "{modified}" }
                }
                if !is_known {
                    p { class: "file-card__note",
                        "Size and date appear once the listing of this folder is loaded."
                    }
                }
            }
            div { class: "file-card__footer",
                button {
                    class: "btn btn--primary",
                    onclick: move |evt| {
                        // a double click is two clicks - download once
                        if crate::utils::is_repeated_click(&evt) {
                            return;
                        }
                        crate::views::download_file(&download_bucket, &download_key);
                    },
                    DownloadIcon {}
                    "Download"
                }
            }
        }
    }
}
