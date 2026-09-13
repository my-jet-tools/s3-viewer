use my_http_utils::macros::{MyHttpInput, MyHttpObjectStructure};
use serde::{Deserialize, Serialize};

pub const LIST_OBJECTS_ROUTE: &str = "/api/objects/v1/list";
pub const DOWNLOAD_OBJECT_ROUTE: &str = "/api/objects/v1/download";

/// One level of a bucket: the "folders" (common prefixes) and files directly under `prefix`,
/// listed with delimiter `/`.
#[derive(MyHttpInput)]
pub struct ListObjectsInputModel {
    #[http_query(
        name = "bucket",
        description = "Bucket name as it is configured in the viewer settings"
    )]
    pub bucket: String,

    #[http_query(
        name = "prefix",
        description = "Folder to list: a key prefix ending with '/'. Absent or empty lists the bucket root"
    )]
    pub prefix: Option<String>,
}

// `prefix` is the full key prefix including the trailing '/', `name` is its last segment
// without the '/', ready to display.
#[derive(Serialize, Deserialize, MyHttpObjectStructure, Clone, Debug, PartialEq)]
pub struct FolderHttpModel {
    pub prefix: String,
    pub name: String,
}

// `key` is the full object key, `name` is the part after the listed prefix.
// `last_modified` is passed through from S3 as is (ISO-8601).
#[derive(Serialize, Deserialize, MyHttpObjectStructure, Clone, Debug, PartialEq)]
pub struct FileHttpModel {
    pub key: String,
    pub name: String,
    pub size: u64,
    pub last_modified: String,
}

// `is_truncated` is true when the server stopped paging before the end of the level
// (a safety cap on huge folders) - the UI must say that the list is incomplete.
#[derive(Serialize, Deserialize, MyHttpObjectStructure, Clone, Debug, PartialEq)]
pub struct ListObjectsResponse {
    pub bucket: String,
    pub prefix: String,
    pub folders: Vec<FolderHttpModel>,
    pub files: Vec<FileHttpModel>,
    pub is_truncated: bool,
}

/// Streams one object back as an attachment. Used by the UI as a top-level browser navigation.
#[derive(MyHttpInput)]
pub struct DownloadObjectInputModel {
    #[http_query(
        name = "bucket",
        description = "Bucket name as it is configured in the viewer settings"
    )]
    pub bucket: String,

    #[http_query(name = "key", description = "Full object key")]
    pub key: String,
}
