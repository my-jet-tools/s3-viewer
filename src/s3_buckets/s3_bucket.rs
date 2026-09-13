use my_s3::S3Client;

use crate::settings::BucketSettingsModel;

pub struct S3Bucket {
    pub name: String,
    pub client: S3Client,
}

impl S3Bucket {
    pub fn new(settings: &BucketSettingsModel) -> Self {
        Self {
            name: settings.name.clone(),
            client: S3Client::new(
                settings.access_key.as_str(),
                settings.secret_key.as_str(),
                settings.region.as_str(),
                settings.endpoint.as_str(),
            ),
        }
    }
}
