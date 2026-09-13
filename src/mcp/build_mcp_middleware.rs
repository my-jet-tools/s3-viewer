use std::sync::Arc;

use mcp_server_middleware::McpMiddleware;

use crate::app::{APP_NAME, APP_VERSION, AppContext};

pub const MCP_PATH: &str = "/mcp";

const MCP_INSTRUCTIONS: &str = "Read-only browser of the S3 buckets configured in s3-viewer. \
Call list_buckets first, then list_objects with a bucket and a folder prefix to walk the tree one level at a time. \
Only names, sizes and modification times are returned - file contents are never downloaded.";

// Read-only by design: the tools only list. Nothing here can download, change or delete an object.
pub fn build_mcp_middleware(app: &Arc<AppContext>) -> McpMiddleware {
    let mut mcp = McpMiddleware::new(MCP_PATH, APP_NAME, APP_VERSION, MCP_INSTRUCTIONS);

    mcp.register_tool_call(Arc::new(super::ListBucketsToolCallHandler::new(
        app.clone(),
    )));

    mcp.register_tool_call(Arc::new(super::ListObjectsToolCallHandler::new(
        app.clone(),
    )));

    mcp
}
