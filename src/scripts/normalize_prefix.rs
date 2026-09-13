// The folder to list, as a key prefix: empty for the bucket root, otherwise ending with '/'.
// A prefix sent without the trailing '/' still means that folder: listing "photos" with delimiter
// '/' would otherwise also match "photos-2024/" and "photos.txt".
pub fn normalize_prefix(prefix: Option<&str>) -> String {
    let Some(prefix) = prefix else {
        return String::new();
    };

    if prefix.is_empty() || prefix.ends_with('/') {
        return prefix.to_string();
    }

    format!("{prefix}/")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn absent_and_empty_prefix_are_the_bucket_root() {
        assert_eq!(normalize_prefix(None), "");
        assert_eq!(normalize_prefix(Some("")), "");
    }

    #[test]
    fn folder_prefix_is_kept_as_is() {
        assert_eq!(normalize_prefix(Some("a/")), "a/");
        assert_eq!(normalize_prefix(Some("a/b c/")), "a/b c/");
    }

    #[test]
    fn missing_trailing_slash_is_added() {
        assert_eq!(normalize_prefix(Some("a/b")), "a/b/");
        assert_eq!(normalize_prefix(Some("фото")), "фото/");
    }
}
