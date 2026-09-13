use my_http_utils::macros::MyHttpObjectStructure;
use serde::{Deserialize, Serialize};

pub const LIST_BUCKETS_ROUTE: &str = "/api/buckets/v1/list";

/// One bucket from the viewer settings. Credentials never leave the server.
#[derive(Serialize, Deserialize, MyHttpObjectStructure, Clone, Debug, PartialEq)]
pub struct BucketHttpModel {
    pub name: String,
}

/// The root level of the tree: the buckets configured in the settings, in settings order.
#[derive(Serialize, Deserialize, MyHttpObjectStructure, Clone, Debug, PartialEq)]
pub struct ListBucketsResponse {
    pub buckets: Vec<BucketHttpModel>,
}
