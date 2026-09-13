use my_s3::S3ObjectInfo;

use crate::models::ListedObject;

impl From<S3ObjectInfo> for ListedObject {
    fn from(object: S3ObjectInfo) -> Self {
        Self {
            key: object.key,
            size: object.size,
            last_modified: object.last_modified,
        }
    }
}
