// Display name of a file: the key minus the listed prefix. With delimiter '/' S3 only returns keys
// that have no further '/' after the prefix, so this is the last segment. A key that does not
// start with the listed prefix can only come from a misbehaving server; it is shown whole.
pub fn get_file_name<'s>(listed_prefix: &str, key: &'s str) -> &'s str {
    key.strip_prefix(listed_prefix).unwrap_or(key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn root_level_file() {
        assert_eq!(get_file_name("", "readme.txt"), "readme.txt");
    }

    #[test]
    fn nested_file() {
        assert_eq!(get_file_name("a/b/", "a/b/c.txt"), "c.txt");
    }

    #[test]
    fn unicode_and_spaces() {
        assert_eq!(
            get_file_name("фото/", "фото/лето 2024.jpg"),
            "лето 2024.jpg"
        );
        assert_eq!(get_file_name("my docs/", "my docs/a b.pdf"), "a b.pdf");
    }

    #[test]
    fn prefix_mismatch_keeps_the_whole_key() {
        assert_eq!(get_file_name("x/", "a/b.txt"), "a/b.txt");
    }
}
