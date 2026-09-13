use std::sync::Arc;

use mcp_server_middleware::{McpToolCall, ToolDefinition};
use my_ai_agent::macros::ApplyJsonSchema;
use serde::{Deserialize, Serialize};

use crate::app::AppContext;

#[derive(ApplyJsonSchema, Debug, Serialize, Deserialize)]
pub struct ListBucketsInputData {}

#[derive(ApplyJsonSchema, Debug, Serialize, Deserialize)]
pub struct ListBucketsMcpResponse {
    #[property(
        description = "Names of the buckets configured in the viewer settings, in settings order. Pass one as bucket to list_objects"
    )]
    pub buckets: Vec<String>,
}

pub struct ListBucketsToolCallHandler {
    app: Arc<AppContext>,
}

impl ListBucketsToolCallHandler {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

impl ToolDefinition for ListBucketsToolCallHandler {
    const FUNC_NAME: &'static str = "list_buckets";

    const DESCRIPTION: &'static str = "Returns the S3 buckets configured in s3-viewer - the root level of the tree. Use a returned name as bucket in list_objects.";
}

#[async_trait::async_trait]
impl McpToolCall<ListBucketsInputData, ListBucketsMcpResponse> for ListBucketsToolCallHandler {
    async fn execute_tool_call(
        &self,
        _model: ListBucketsInputData,
    ) -> Result<ListBucketsMcpResponse, String> {
        Ok(crate::flows::list_buckets(&self.app).into())
    }
}
