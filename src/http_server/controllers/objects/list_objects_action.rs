use std::sync::Arc;

use my_http_server::macros::http_route;
use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput};
use rest_api_shared::{ListObjectsInputModel, ListObjectsResponse};

use crate::app::AppContext;

// route == rest_api_shared::LIST_OBJECTS_ROUTE
#[http_route(
    method: "GET",
    route: "/api/objects/v1/list",
    controller: "Objects",
    summary: "List one level of a bucket",
    description: "The folders (common prefixes) and files directly under the prefix, listed with delimiter '/'. Absent or empty prefix lists the bucket root",
    input_data: "ListObjectsInputModel",
    result: [
        {status_code: 200, description: "One level of the bucket", model: "ListObjectsResponse"},
        {status_code: 404, description: "The bucket is not configured, or does not exist in S3"},
        {status_code: 500, description: "S3 request failed"},
    ]
)]
pub struct ListObjectsAction {
    app: Arc<AppContext>,
}

impl ListObjectsAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

// The return type is dictated by `http_route`: its generated trait impl returns this fn's
// `Result<HttpOkResult, HttpFailResult>` as is, so the framework's error type can not be boxed on
// our side (see `result_large_err` in Cargo.toml). Everything below the handler returns
// `S3ViewerError`.
async fn handle_request(
    action: &ListObjectsAction,
    input_data: ListObjectsInputModel,
    _ctx: &mut HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let response = crate::flows::list_objects(&action.app, input_data).await?;

    HttpOutput::as_json(response).into_ok_result(false)
}
