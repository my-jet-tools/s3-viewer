use rest_api_shared::{ListObjectsInputModel, ListObjectsResponse};

use crate::app::AppContext;
use crate::models::S3ViewerError;

pub async fn list_objects(
    app: &AppContext,
    input: ListObjectsInputModel,
) -> Result<ListObjectsResponse, S3ViewerError> {
    let bucket = app
        .s3_buckets
        .get(input.bucket.as_str())
        .ok_or(S3ViewerError::BucketIsNotConfigured)?;

    let prefix = crate::scripts::normalize_prefix(input.prefix.as_deref());

    let level =
        crate::scripts::list_one_level(&bucket.client, bucket.name.as_str(), prefix.as_str())
            .await?;

    Ok(crate::scripts::build_list_objects_response(
        bucket.name.as_str(),
        prefix,
        level,
    ))
}
