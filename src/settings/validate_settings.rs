use std::collections::HashSet;

use super::{SettingsModel, SettingsValidationError};

// The bucket name is the key the UI addresses a bucket by, so it has to be present and unique.
pub fn validate_settings(settings: &SettingsModel) -> Result<(), SettingsValidationError> {
    if settings.buckets.is_empty() {
        return Err(SettingsValidationError::NoBuckets);
    }

    let mut names = HashSet::new();

    for (index, bucket) in settings.buckets.iter().enumerate() {
        if bucket.name.trim().is_empty() {
            return Err(SettingsValidationError::EmptyBucketName { index });
        }

        if !names.insert(bucket.name.as_str()) {
            return Err(SettingsValidationError::DuplicateBucketName {
                name: bucket.name.clone(),
            });
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::DEFAULT_HTTP_PORT;

    fn parse(yaml: &str) -> SettingsModel {
        serde_yaml::from_str(yaml).unwrap()
    }

    fn bucket_yaml(name: &str) -> String {
        format!(
            "  - name: {name}\n    endpoint: https://e\n    region: r\n    access_key: a\n    secret_key: s\n"
        )
    }

    #[test]
    fn example_settings_file_is_valid() {
        let settings = parse(include_str!("../../settings.example.yaml"));

        assert_eq!(settings.get_http_port(), 8000);
        assert_eq!(settings.buckets.len(), 1);
        assert_eq!(validate_settings(&settings), Ok(()));
    }

    #[test]
    fn http_port_is_optional() {
        let settings = parse(&format!("buckets:\n{}", bucket_yaml("b1")));

        assert_eq!(settings.get_http_port(), DEFAULT_HTTP_PORT);
        assert_eq!(validate_settings(&settings), Ok(()));
    }

    #[test]
    fn empty_bucket_list_is_rejected() {
        let settings = parse("http_port: 9000\nbuckets: []\n");

        assert_eq!(
            validate_settings(&settings),
            Err(SettingsValidationError::NoBuckets)
        );
    }

    #[test]
    fn empty_bucket_name_is_rejected() {
        let settings = parse(&format!(
            "buckets:\n{}{}",
            bucket_yaml("b1"),
            bucket_yaml("\"\"")
        ));

        assert_eq!(
            validate_settings(&settings),
            Err(SettingsValidationError::EmptyBucketName { index: 1 })
        );
    }

    #[test]
    fn duplicate_bucket_names_are_rejected() {
        let settings = parse(&format!(
            "buckets:\n{}{}{}",
            bucket_yaml("b1"),
            bucket_yaml("b2"),
            bucket_yaml("b1")
        ));

        assert_eq!(
            validate_settings(&settings),
            Err(SettingsValidationError::DuplicateBucketName {
                name: "b1".to_string()
            })
        );
    }
}
