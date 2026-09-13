// The part of an S3 object listing entry the viewer uses. Kept separate from my-s3's
// `S3ObjectInfo` so the response-building logic does not depend on that struct's exact shape.
pub struct ListedObject {
    pub key: String,
    pub size: u64,
    pub last_modified: String,
}
