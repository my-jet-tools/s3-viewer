use rest_api_shared::{BucketHttpModel, ListBucketsResponse};

use crate::app::AppContext;

pub fn list_buckets(app: &AppContext) -> ListBucketsResponse {
    ListBucketsResponse {
        buckets: app
            .s3_buckets
            .get_all()
            .iter()
            .map(BucketHttpModel::from)
            .collect(),
    }
}
