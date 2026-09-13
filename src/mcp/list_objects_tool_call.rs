use std::sync::Arc;

use mcp_server_middleware::{McpToolCall, ToolDefinition};
use my_ai_agent::macros::ApplyJsonSchema;
use rest_api_shared::ListObjectsInputModel;
use serde::{Deserialize, Serialize};

use crate::app::AppContext;

#[derive(ApplyJsonSchema, Debug, Serialize, Deserialize)]
pub struct ListObjectsInputData {
    #[property(description = "Bucket name, as returned by list_buckets")]
    pub bucket: String,

    #[property(
        description = "Folder to list: a key prefix ending with '/', for example 'docs/2024/'. Omit it or pass an empty string for the bucket root"
    )]
    pub prefix: Option<String>,
}

#[derive(ApplyJsonSchema, Debug, Serialize, Deserialize)]
pub struct McpFolderModel {
    #[property(
        description = "Full prefix of the folder including the trailing '/'. Pass it as prefix to list_objects to go inside"
    )]
    pub prefix: String,

    #[property(description = "Folder name: the last segment, without '/'")]
    pub name: String,
}

// Not `key`: the ApplyJsonSchema derive (my-ai-agent 0.1.0) generates a deserializer whose loop
// binds its own `key` / `value` locals, which shadow a field of the same name and break the build.
#[derive(ApplyJsonSchema, Debug, Serialize, Deserialize)]
pub struct McpFileModel {
    #[property(description = "Full object key")]
    pub object_key: String,

    #[property(description = "File name relative to the listed folder")]
    pub name: String,

    #[property(description = "Size in bytes")]
    pub size: u64,

    #[property(description = "Last modification time as reported by S3 (ISO-8601)")]
    pub last_modified: String,
}

#[derive(ApplyJsonSchema, Debug, Serialize, Deserialize)]
pub struct ListObjectsMcpResponse {
    #[property(description = "Bucket that was listed")]
    pub bucket: String,

    #[property(description = "Folder prefix that was listed. Empty string is the bucket root")]
    pub prefix: String,

    #[property(description = "Sub-folders directly under the prefix, sorted by name")]
    pub folders: Vec<McpFolderModel>,

    #[property(
        description = "Files directly under the prefix, sorted by name. Only metadata - the content is never downloaded"
    )]
    pub files: Vec<McpFileModel>,

    #[property(
        description = "True when the folder is so large that listing stopped at the safety cap and the lists above are incomplete"
    )]
    pub is_truncated: bool,
}

pub struct ListObjectsToolCallHandler {
    app: Arc<AppContext>,
}

impl ListObjectsToolCallHandler {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

impl ToolDefinition for ListObjectsToolCallHandler {
    const FUNC_NAME: &'static str = "list_objects";

    const DESCRIPTION: &'static str = "Lists ONE level of a bucket without downloading anything: the sub-folders and the files (key, size, last modified) directly under a folder prefix. Walk deeper by calling it again with a returned folder prefix.";
}

#[async_trait::async_trait]
impl McpToolCall<ListObjectsInputData, ListObjectsMcpResponse> for ListObjectsToolCallHandler {
    async fn execute_tool_call(
        &self,
        model: ListObjectsInputData,
    ) -> Result<ListObjectsMcpResponse, String> {
        let input = ListObjectsInputModel {
            bucket: model.bucket,
            prefix: model.prefix,
        };

        let response = crate::flows::list_objects(&self.app, input).await?;

        Ok(response.into())
    }
}
