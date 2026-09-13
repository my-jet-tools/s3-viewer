use std::collections::HashSet;

use super::{
    DEFAULT_HTTP_PORT, S3ConnString, SettingsModel, SettingsValidationError, SettingsYamlModel,
};

// Parses every bucket connection string and checks the bucket names: the name is the key the UI
// and the MCP tools address a bucket by, so it has to be unique.
pub fn validate_settings(
    settings: SettingsYamlModel,
) -> Result<SettingsModel, SettingsValidationError> {
    if settings.buckets.is_empty() {
        return Err(SettingsValidationError::NoBuckets);
    }

    let mut buckets = Vec::with_capacity(settings.buckets.len());
    let mut names = HashSet::new();

    for (index, conn_string) in settings.buckets.into_iter().enumerate() {
        let bucket = S3ConnString::parse(conn_string.as_str())
            .map_err(|err| SettingsValidationError::InvalidBucketConnString { index, err })?;

        if !names.insert(bucket.bucket.clone()) {
            return Err(SettingsValidationError::DuplicateBucketName {
                name: bucket.bucket,
            });
        }

        buckets.push(bucket);
    }

    Ok(SettingsModel {
        http_port: settings.http_port.unwrap_or(DEFAULT_HTTP_PORT),
        buckets,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::S3ConnStringError;

    const SECRET: &str = "TopSecretValue";

    fn validate(yaml: &str) -> Result<SettingsModel, SettingsValidationError> {
        validate_settings(serde_yaml::from_str(yaml).unwrap())
    }

    fn validate_ok(yaml: &str) -> SettingsModel {
        match validate(yaml) {
            Ok(settings) => settings,
            Err(err) => panic!("settings should be valid, got: {err}"),
        }
    }

    fn bucket_line(bucket: &str) -> String {
        format!(
            "  - \"Endpoint=https://e;Region=r;AccessKey=a;SecretKey={SECRET};Bucket={bucket}\"\n"
        )
    }

    #[test]
    fn example_settings_file_is_valid() {
        let settings = validate_ok(include_str!("../../settings.example.yaml"));

        assert_eq!(settings.get_http_port(), 8000);
        assert_eq!(settings.buckets.len(), 1);
        assert_eq!(settings.buckets[0].bucket, "my-bucket");
        assert!(!settings.buckets[0].debug);
    }

    #[test]
    fn http_port_is_optional() {
        let settings = validate_ok(&format!("buckets:\n{}", bucket_line("b1")));

        assert_eq!(settings.get_http_port(), DEFAULT_HTTP_PORT);
    }

    #[test]
    fn buckets_keep_settings_order() {
        let settings = validate_ok(&format!(
            "buckets:\n{}{}{}",
            bucket_line("zeta"),
            bucket_line("alpha"),
            bucket_line("mid")
        ));

        let names: Vec<&str> = settings
            .buckets
            .iter()
            .map(|bucket| bucket.bucket.as_str())
            .collect();

        assert_eq!(names, vec!["zeta", "alpha", "mid"]);
    }

    #[test]
    fn empty_bucket_list_is_rejected() {
        assert_eq!(
            validate("http_port: 9000\nbuckets: []\n").err(),
            Some(SettingsValidationError::NoBuckets)
        );
    }

    #[test]
    fn an_invalid_connection_string_names_its_position() {
        let yaml = format!(
            "buckets:\n{}  - \"Endpoint=https://e;Region=r;AccessKey=a;SecretKey={SECRET}\"\n",
            bucket_line("b1")
        );

        let err = validate(&yaml).err();

        assert_eq!(
            err,
            Some(SettingsValidationError::InvalidBucketConnString {
                index: 1,
                err: S3ConnStringError::MissingKey { key: "Bucket" }
            })
        );

        let message = err.unwrap().to_string();
        assert!(message.contains("bucket #2"), "{message}");
        assert!(!message.contains(SECRET), "{message}");
    }

    #[test]
    fn duplicate_bucket_names_are_rejected() {
        let yaml = format!(
            "buckets:\n{}{}{}",
            bucket_line("b1"),
            bucket_line("b2"),
            bucket_line("b1")
        );

        assert_eq!(
            validate(&yaml).err(),
            Some(SettingsValidationError::DuplicateBucketName {
                name: "b1".to_string()
            })
        );
    }

    #[test]
    fn debug_reaches_the_bucket() {
        let settings = validate_ok(
            "buckets:\n  - \"Endpoint=https://e;Region=r;AccessKey=a;SecretKey=s;Bucket=b1;Debug=1\"\n",
        );

        assert!(settings.buckets[0].debug);
    }
}
