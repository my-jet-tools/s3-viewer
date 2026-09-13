use std::sync::Arc;

use my_http_server::macros::http_route;
use my_http_server::{HttpContext, HttpFailResult, HttpOkResult};
use rest_api_shared::DownloadObjectInputModel;

use crate::app::AppContext;

// route == rest_api_shared::DOWNLOAD_OBJECT_ROUTE
#[http_route(
    method: "GET",
    route: "/api/objects/v1/download",
    controller: "Objects",
    summary: "Download an object",
    description: "Streams the object back as an attachment named after the last segment of its key",
    input_data: "DownloadObjectInputModel",
    result: [
        {status_code: 200, description: "The object content, streamed"},
        {status_code: 404, description: "The bucket is not configured, or the bucket / key does not exist in S3"},
        {status_code: 500, description: "S3 request failed"},
    ]
)]
pub struct DownloadObjectAction {
    app: Arc<AppContext>,
}

impl DownloadObjectAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

// The return type is dictated by `http_route`: its generated trait impl returns this fn's
// `Result<HttpOkResult, HttpFailResult>` as is, so the framework's error type can not be boxed on
// our side. Everything below the handler returns `S3ViewerError`.
#[allow(clippy::result_large_err)]
async fn handle_request(
    action: &DownloadObjectAction,
    input_data: DownloadObjectInputModel,
    _ctx: &mut HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    crate::flows::download_object(&action.app, input_data)
        .await?
        .get_result()
}
