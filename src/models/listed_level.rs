use super::ListedObject;

// Everything S3 returned for one level (one prefix, delimiter '/'), all pages concatenated.
pub struct ListedLevel {
    pub common_prefixes: Vec<String>,
    pub objects: Vec<ListedObject>,
    // The listing stopped while S3 still had a continuation token: the page cap was reached, or
    // S3 answered with the same token it was sent.
    pub is_truncated: bool,
}
