// The zero-length object whose key equals the listed prefix ("a/b/"), created by consoles and
// tools to make an empty folder exist. It is the folder itself, not a file inside it.
pub fn is_folder_marker(listed_prefix: &str, key: &str, size: u64) -> bool {
    size == 0 && !listed_prefix.is_empty() && key == listed_prefix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_length_object_named_like_the_prefix_is_a_marker() {
        assert!(is_folder_marker("a/b/", "a/b/", 0));
    }

    #[test]
    fn regular_files_are_not_markers() {
        assert!(!is_folder_marker("a/b/", "a/b/c.txt", 0));
        assert!(!is_folder_marker("a/b/", "a/b/c.txt", 10));
    }

    #[test]
    fn object_with_content_is_not_skipped() {
        assert!(!is_folder_marker("a/b/", "a/b/", 10));
    }

    #[test]
    fn root_level_has_no_marker() {
        assert!(!is_folder_marker("", "", 0));
    }
}
