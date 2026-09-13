use std::time::Duration;

use flurl::my_http_utils::UrlBuilder;
use flurl::my_http_utils::schema::client::THttpRequestBuilder;
use flurl::{FlUrl, HttpVerb};
use rest_api_shared::{
    DOWNLOAD_OBJECT_ROUTE, DownloadObjectInputModel, LIST_OBJECTS_ROUTE, ListObjectsInputModel,
    ListObjectsResponse,
};

use crate::models::RequestError;

// The server pages a big folder against S3 before it answers; FlUrl's 10s default would cut it off.
const LIST_OBJECTS_TIMEOUT: Duration = Duration::from_secs(60);

// Only a carrier for `UrlBuilder`: the host is thrown away, only the path and query are kept.
const PLACEHOLDER_ORIGIN: &str = "http://placeholder";

/// One level of a bucket: the folders and files directly under `prefix` ("" = the bucket root).
pub async fn list_objects(bucket: &str, prefix: &str) -> Result<ListObjectsResponse, RequestError> {
    let input = ListObjectsInputModel {
        bucket: bucket.to_string(),
        prefix: if prefix.is_empty() {
            None
        } else {
            Some(prefix.to_string())
        },
    };

    let response = FlUrl::new(LIST_OBJECTS_ROUTE)
        .set_timeout(LIST_OBJECTS_TIMEOUT)
        .execute_request(HttpVerb::Get, input)
        .await;

    super::handle_http_response(response).await
}

/// The RELATIVE url of an object download: `/api/objects/v1/download?bucket=..&key=..`.
///
/// The browser navigates to it (FlUrl never runs), so the query is filled by the shared
/// `DownloadObjectInputModel` itself - the parameter names cannot drift from what the server
/// parses. `UrlBuilder` needs an absolute url to treat the input as TCP (a leading '/' reads as a
/// unix socket path), hence the placeholder origin that `get_path_and_query` drops again.
pub fn download_url(bucket: &str, key: &str) -> Result<String, RequestError> {
    let input = DownloadObjectInputModel {
        bucket: bucket.to_string(),
        key: key.to_string(),
    };

    let mut url = UrlBuilder::new(&format!("{PLACEHOLDER_ORIGIN}{DOWNLOAD_OBJECT_ROUTE}"));

    input.fill_url(&mut url).map_err(|err| RequestError {
        message: format!("{err:?}"),
    })?;

    Ok(url.get_path_and_query())
}
