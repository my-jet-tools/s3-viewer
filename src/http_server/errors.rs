use my_http_server::HttpFailResult;
use my_logger::LogEventCtx;
use my_s3::S3Error;

use crate::models::S3ViewerError;

const ACCESS_DENIED_STATUS_CODE: u16 = 403;

// The one place a bucket failure becomes an HTTP answer. Actions and flows just use `?`.
impl From<S3ViewerError> for HttpFailResult {
    fn from(err: S3ViewerError) -> Self {
        match err {
            S3ViewerError::BucketIsNotConfigured => {
                HttpFailResult::as_not_found("Bucket is not configured", false)
            }
            S3ViewerError::S3(err) => from_s3_error(&err),
        }
    }
}

// Not-found answers are 404; everything else is a 500 with a short fixed text. The S3Error itself
// (transport error or the XML body S3 answered with - endpoint, bucket, key, request ids) goes to
// the log only, never to the client.
fn from_s3_error(err: &S3Error) -> HttpFailResult {
    if err.is_bucket_not_found() {
        return HttpFailResult::as_not_found("Bucket is not found in S3", false);
    }

    if err.is_key_not_found() {
        return HttpFailResult::as_not_found("Object is not found", false);
    }

    my_logger::LOGGER.write_error("s3_request", err.to_string(), LogEventCtx::new());

    if err.get_status_code() == Some(ACCESS_DENIED_STATUS_CODE) {
        return HttpFailResult::as_fatal_error("Access to the bucket is denied by S3");
    }

    HttpFailResult::as_fatal_error("S3 request failed")
}

#[cfg(test)]
mod tests {
    use my_http_server::my_hyper_utils::MyHttpResponse;
    use my_http_server::{HttpFailResult, HttpOutput};
    use my_s3::S3Error;

    use crate::models::S3ViewerError;

    fn status_code_of(err: S3ViewerError) -> u16 {
        let fail: HttpFailResult = err.into();
        let response: MyHttpResponse = fail.into();
        response.status().as_u16()
    }

    fn text_of(err: S3ViewerError) -> String {
        let fail: HttpFailResult = err.into();

        let HttpOutput::Content { content, .. } = fail.output else {
            panic!("an S3 failure must be answered with a text body");
        };

        String::from_utf8(content).expect("the error text is UTF-8")
    }

    #[test]
    fn unknown_bucket_is_404() {
        assert_eq!(status_code_of(S3ViewerError::BucketIsNotConfigured), 404);
    }

    #[test]
    fn missing_bucket_or_key_in_s3_is_404() {
        assert_eq!(status_code_of(S3Error::BucketNotFound.into()), 404);
        assert_eq!(status_code_of(S3Error::KeyNotFound.into()), 404);
    }

    #[test]
    fn other_s3_errors_are_500() {
        assert_eq!(
            status_code_of(S3Error::Other("boom".to_string()).into()),
            500
        );
        assert_eq!(
            status_code_of(
                S3Error::UnexpectedStatusCode {
                    status_code: 403,
                    error_code: Some("AccessDenied".to_string()),
                    body: String::new(),
                }
                .into()
            ),
            500
        );
    }

    #[test]
    fn s3_error_details_do_not_reach_the_client() {
        let text = text_of(
            S3Error::UnexpectedStatusCode {
                status_code: 500,
                error_code: Some("InternalError".to_string()),
                body: "<Error><Resource>/secret-bucket/key</Resource><RequestId>R1</RequestId></Error>"
                    .to_string(),
            }
            .into(),
        );

        assert_eq!(text, "S3 request failed");
    }

    #[test]
    fn access_denied_gets_its_own_short_text() {
        let text = text_of(
            S3Error::UnexpectedStatusCode {
                status_code: 403,
                error_code: Some("AccessDenied".to_string()),
                body: "<Error><RequestId>R1</RequestId></Error>".to_string(),
            }
            .into(),
        );

        assert_eq!(text, "Access to the bucket is denied by S3");
    }
}
