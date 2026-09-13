// Display name of a sub-folder: the common prefix minus the listed prefix minus the trailing '/'.
// A common prefix that does not start with the listed prefix can only come from a misbehaving
// server; it is shown whole rather than dropped.
pub fn get_folder_name<'s>(listed_prefix: &str, common_prefix: &'s str) -> &'s str {
    let relative = common_prefix
        .strip_prefix(listed_prefix)
        .unwrap_or(common_prefix);

    relative.strip_suffix('/').unwrap_or(relative)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn root_level_folder() {
        assert_eq!(get_folder_name("", "photos/"), "photos");
    }

    #[test]
    fn nested_folder() {
        assert_eq!(get_folder_name("a/b/", "a/b/c/"), "c");
    }

    #[test]
    fn unicode_and_spaces() {
        assert_eq!(
            get_folder_name("документы/", "документы/мой отчёт/"),
            "мой отчёт"
        );
        assert_eq!(get_folder_name("", "my folder/"), "my folder");
    }

    #[test]
    fn empty_segment_of_a_double_slash() {
        assert_eq!(get_folder_name("a/", "a//"), "");
    }

    #[test]
    fn prefix_mismatch_keeps_the_whole_prefix() {
        assert_eq!(get_folder_name("x/", "a/b/"), "a/b");
    }
}
