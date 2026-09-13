use rest_api_shared::{FileHttpModel, FolderHttpModel, ListBucketsResponse, ListObjectsResponse};

use crate::mcp::{ListBucketsMcpResponse, ListObjectsMcpResponse, McpFileModel, McpFolderModel};

// The MCP tools answer with the same listing the REST API serves; only the schema derive differs
// (ApplyJsonSchema instead of MyHttpObjectStructure), so the fields map one to one.

impl From<ListBucketsResponse> for ListBucketsMcpResponse {
    fn from(src: ListBucketsResponse) -> Self {
        Self {
            buckets: src.buckets.into_iter().map(|bucket| bucket.name).collect(),
        }
    }
}

impl From<ListObjectsResponse> for ListObjectsMcpResponse {
    fn from(src: ListObjectsResponse) -> Self {
        Self {
            bucket: src.bucket,
            prefix: src.prefix,
            folders: src.folders.into_iter().map(McpFolderModel::from).collect(),
            files: src.files.into_iter().map(McpFileModel::from).collect(),
            is_truncated: src.is_truncated,
        }
    }
}

impl From<FolderHttpModel> for McpFolderModel {
    fn from(src: FolderHttpModel) -> Self {
        Self {
            prefix: src.prefix,
            name: src.name,
        }
    }
}

impl From<FileHttpModel> for McpFileModel {
    fn from(src: FileHttpModel) -> Self {
        Self {
            object_key: src.key,
            name: src.name,
            size: src.size,
            last_modified: src.last_modified,
        }
    }
}
