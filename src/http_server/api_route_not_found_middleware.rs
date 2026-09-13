use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpPath, HttpServerMiddleware};

const API_SEGMENT: &str = "api";

// Registered after the controllers and before the static files. Every real API route has already
// been answered by then, so whatever reaches this point under /api is a route that does not exist.
// Without it StaticFilesMiddleware would answer /api/typo with index.html and a 200, and the UI
// would try to parse HTML as JSON.
pub struct ApiRouteNotFoundMiddleware;

#[async_trait::async_trait]
impl HttpServerMiddleware for ApiRouteNotFoundMiddleware {
    async fn handle_request(
        &self,
        ctx: &mut HttpContext,
    ) -> Option<Result<HttpOkResult, HttpFailResult>> {
        if !is_api_path(&ctx.request.http_path) {
            return None;
        }

        Some(Err(api_route_not_found()))
    }
}

// The first path segment is exactly `api`, in any case. `/api`, `/api/` and `/API/x` match;
// `/apis`, `/api-docs` and `/assets/api` do not.
fn is_api_path(path: &HttpPath) -> bool {
    path.has_value_at_index_case_insensitive(0, API_SEGMENT)
}

// Not written to the log: anyone can generate unknown URLs.
fn api_route_not_found() -> HttpFailResult {
    HttpFailResult::as_not_found("API route not found", false)
}

#[cfg(test)]
mod tests {
    use my_http_server::my_hyper_utils::MyHttpResponse;
    use my_http_server::{HttpOutput, HttpPath};

    fn is_api(path: &str) -> bool {
        super::is_api_path(&HttpPath::from_str(path))
    }

    #[test]
    fn paths_under_api_match() {
        assert!(is_api("/api"));
        assert!(is_api("/api/"));
        assert!(is_api("/api/typo"));
        assert!(is_api("/api/buckets/v1/list"));
        assert!(is_api("/api/objects/v1/unknown/deeper"));
    }

    #[test]
    fn api_segment_is_case_insensitive() {
        assert!(is_api("/API"));
        assert!(is_api("/Api/buckets/v1/list"));
        assert!(is_api("/aPi/typo"));
    }

    #[test]
    fn only_the_whole_first_segment_counts() {
        assert!(!is_api("/"));
        assert!(!is_api("/apis"));
        assert!(!is_api("/api-docs"));
        assert!(!is_api("/apiv1/list"));
        assert!(!is_api("/ap"));
        assert!(!is_api("/swagger"));
        assert!(!is_api("/index.html"));
        assert!(!is_api("/assets/api"));
        assert!(!is_api("/buckets/api/list"));
    }

    #[test]
    fn answer_is_404_with_a_short_text() {
        let response: MyHttpResponse = super::api_route_not_found().into();
        assert_eq!(response.status().as_u16(), 404);

        let HttpOutput::Content { content, .. } = super::api_route_not_found().output else {
            panic!("the 404 must be answered with a text body");
        };

        assert_eq!(String::from_utf8(content).unwrap(), "API route not found");
    }
}
