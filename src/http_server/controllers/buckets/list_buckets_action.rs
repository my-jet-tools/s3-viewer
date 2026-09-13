use std::sync::Arc;

use my_http_server::macros::http_route;
use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput};
use rest_api_shared::ListBucketsResponse;

use crate::app::AppContext;

// route == rest_api_shared::LIST_BUCKETS_ROUTE
#[http_route(
    method: "GET",
    route: "/api/buckets/v1/list",
    controller: "Buckets",
    summary: "List configured buckets",
    description: "The root level of the tree: the buckets configured in the settings file, in settings order",
    result: [
        {status_code: 200, description: "Configured buckets", model: "ListBucketsResponse"},
    ]
)]
pub struct ListBucketsAction {
    app: Arc<AppContext>,
}

impl ListBucketsAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

// The shape is dictated by `http_route`: its generated trait impl awaits
// `handle_request(self, ctx)` and returns its `Result<HttpOkResult, HttpFailResult>` as is. So the
// fn must be async although this handler has nothing to await, and the error type is the
// framework's, which can not be boxed on our side.
#[allow(clippy::unused_async, clippy::result_large_err)]
async fn handle_request(
    action: &ListBucketsAction,
    _ctx: &mut HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let response = crate::flows::list_buckets(&action.app);

    HttpOutput::as_json(response).into_ok_result(false)
}
