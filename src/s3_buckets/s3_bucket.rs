use my_s3::S3Client;

use crate::settings::S3ConnString;

pub struct S3Bucket {
    pub name: String,
    pub client: S3Client,
}

impl S3Bucket {
    pub fn new(settings: &S3ConnString) -> Self {
        let client = S3Client::new(
            settings.access_key.as_str(),
            settings.secret_key.as_str(),
            settings.region.as_str(),
            settings.endpoint.as_str(),
        );

        // `Debug=1` in the connection string: every request of this bucket is traced to stdout.
        let client = if settings.debug {
            client.debug_to_console()
        } else {
            client
        };

        Self {
            name: settings.bucket.clone(),
            client,
        }
    }
}
