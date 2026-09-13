use flurl::{EmptyRequestModel, FlUrl, HttpVerb};
use rest_api_shared::{BucketHttpModel, LIST_BUCKETS_ROUTE, ListBucketsResponse};

use crate::models::RequestError;

/// The root level of the tree: the buckets configured in the server settings, in settings order.
pub async fn get_buckets() -> Result<Vec<BucketHttpModel>, RequestError> {
    let response = FlUrl::new(LIST_BUCKETS_ROUTE)
        .execute_request(HttpVerb::Get, EmptyRequestModel)
        .await;

    let listing: ListBucketsResponse = super::handle_http_response(response).await?;

    Ok(listing.buckets)
}
