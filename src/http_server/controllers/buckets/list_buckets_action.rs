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

// `http_route` generates `handle_request(self, ctx).await`, so it needs something it can await, not
// an `async fn`. This handler has nothing to await (the buckets come from settings), so it is a
// plain fn that hands back an already completed future.
fn handle_request(
    action: &ListBucketsAction,
    _ctx: &mut HttpContext,
) -> std::future::Ready<Result<HttpOkResult, HttpFailResult>> {
    let response = crate::flows::list_buckets(&action.app);

    std::future::ready(HttpOutput::as_json(response).into_ok_result(false))
}
