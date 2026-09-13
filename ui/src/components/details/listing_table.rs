use dioxus::prelude::*;
use rest_api_shared::{FileHttpModel, FolderHttpModel, ListObjectsResponse};

use crate::icons::{DownloadIcon, FileIcon, FolderIcon, WarningIcon};
use crate::states::{ExplorerState, LevelKey};

/// The loaded listing of the selected level: folders first, then files.
/// Double click on a folder opens it in the tree, double click on a file downloads it.
pub fn render_listing_table(
    cs: Signal<ExplorerState>,
    key: &LevelKey,
    listing: &ListObjectsResponse,
) -> Element {
    let truncated_notice = if listing.is_truncated {
        rsx! {
            div { class: "notice notice--warn",
                WarningIcon {}
                span {
                    "The listing is incomplete: the server stopped paging before the end of this folder, so not every item is shown."
                }
            }
        }
    } else {
        rsx! {}
    };

    if listing.folders.is_empty() && listing.files.is_empty() {
        return rsx! {
            {truncated_notice}
            div { class: "placeholder",
                div { class: "placeholder__title", "Empty" }
                div { class: "placeholder__text", "Nothing is stored under this prefix." }
            }
        };
    }

    let folder_rows = listing
        .folders
        .iter()
        .map(|folder| render_folder_line(cs, &key.bucket, folder));
    let file_rows = listing
        .files
        .iter()
        .map(|file| render_file_line(&key.bucket, file));

    rsx! {
        {truncated_notice}
        div { class: "listing-card",
            table { class: "listing",
                colgroup {
                    col {}
                    col { class: "listing__col-size" }
                    col { class: "listing__col-date" }
                    col { class: "listing__col-actions" }
                }
                thead {
                    tr {
                        th { "Name" }
                        th { class: "num", "Size" }
                        th { "Last modified" }
                        th {}
                    }
                }
                tbody {
                    {folder_rows}
                    {file_rows}
                }
            }
        }
    }
}

fn render_folder_line(cs: Signal<ExplorerState>, bucket: &str, folder: &FolderHttpModel) -> Element {
    let open_key = LevelKey::folder(bucket, &folder.prefix);

    rsx! {
        tr {
            key: "{folder.prefix}",
            class: "listing__row",
            ondoubleclick: move |_| crate::views::reveal_level(cs, open_key.clone()),
            td {
                div { class: "listing__name", title: "{folder.name}",
                    span { class: "listing__icon is-folder", FolderIcon {} }
                    span { class: "listing__label", "{folder.name}" }
                }
            }
            td { class: "num muted", "—" }
            td { class: "muted", "—" }
            td {}
        }
    }
}

fn render_file_line(bucket: &str, file: &FileHttpModel) -> Element {
    let size = crate::utils::format_size(file.size);
    let exact_size = crate::utils::format_bytes_exact(file.size);
    let modified = crate::utils::format_last_modified(&file.last_modified);

    let row_bucket = bucket.to_string();
    let row_key = file.key.clone();
    let button_bucket = bucket.to_string();
    let button_key = file.key.clone();

    rsx! {
        tr {
            key: "{file.key}",
            class: "listing__row",
            ondoubleclick: move |_| crate::views::download_file(&row_bucket, &row_key),
            td {
                div { class: "listing__name", title: "{file.name}",
                    span { class: "listing__icon is-file", FileIcon {} }
                    span { class: "listing__label", "{file.name}" }
                }
            }
            td { class: "num", title: "{exact_size} bytes", "{size}" }
            td { title: "{file.last_modified}", "{modified}" }
            td { class: "listing__actions",
                button {
                    class: "icon-btn",
                    title: "Download",
                    onclick: move |evt| {
                        evt.stop_propagation();
                        // a double click is two clicks - download once
                        if crate::utils::is_repeated_click(&evt) {
                            return;
                        }
                        crate::views::download_file(&button_bucket, &button_key);
                    },
                    ondoubleclick: move |evt| evt.stop_propagation(),
                    DownloadIcon {}
                }
            }
        }
    }
}
