use my_s3::{S3Client, S3Error, S3ListObjectsRequest};

use crate::models::{ListedLevel, ListedObject};

// S3's own maximum page size.
pub const LIST_PAGE_SIZE: u32 = 1000;

// Safety cap for huge folders: at most 100 pages x 1000 keys are read for one level. When the cap
// stops the listing, the level is reported as truncated.
pub const MAX_LIST_PAGES_PER_LEVEL: usize = 100;

const FOLDER_DELIMITER: &str = "/";

// Lists ONE level of a bucket: the common prefixes and objects directly under `prefix`
// (empty = bucket root), following continuation tokens until S3 has no more pages.
//
// A server that answers with the very token it was just sent would serve the same page again and
// again: the listing stops there and the level is reported as truncated, so no entry is repeated.
pub async fn list_one_level(
    client: &S3Client,
    bucket_name: &str,
    prefix: &str,
) -> Result<ListedLevel, S3Error> {
    let mut result = ListedLevel {
        common_prefixes: Vec::new(),
        objects: Vec::new(),
        is_truncated: false,
    };

    let mut continuation_token: Option<String> = None;

    for _ in 0..MAX_LIST_PAGES_PER_LEVEL {
        let request = S3ListObjectsRequest {
            prefix: if prefix.is_empty() {
                None
            } else {
                Some(prefix)
            },
            delimiter: Some(FOLDER_DELIMITER),
            continuation_token: continuation_token.as_deref(),
            max_keys: Some(LIST_PAGE_SIZE),
        };

        let page = client.list_objects_v2(bucket_name, request).await?;

        result.common_prefixes.extend(page.common_prefixes);
        result
            .objects
            .extend(page.objects.into_iter().map(ListedObject::from));

        match page.next_continuation_token {
            None => return Ok(result),
            Some(token) if continuation_token.as_deref() == Some(token.as_str()) => break,
            Some(token) => continuation_token = Some(token),
        }
    }

    result.is_truncated = true;

    Ok(result)
}
