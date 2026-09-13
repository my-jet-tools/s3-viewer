// Expands a LEADING `~` of a settings path into `home`: the path `~` itself, or `~/...`.
// A `~` anywhere else is an ordinary file-name character and stays; so does `~user/...`, which
// would need a passwd lookup. Without a known home directory the path is used as is.
pub fn expand_home_dir(path: &str, home: Option<&str>) -> String {
    let Some(home) = home else {
        return path.to_string();
    };

    if path == "~" {
        return home.to_string();
    }

    match path.strip_prefix("~/") {
        Some(rest) => format!("{}/{rest}", home.trim_end_matches('/')),
        None => path.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leading_tilde_is_the_home_dir() {
        assert_eq!(
            expand_home_dir("~/.s3-viewer", Some("/home/user")),
            "/home/user/.s3-viewer"
        );
        assert_eq!(expand_home_dir("~", Some("/home/user")), "/home/user");
    }

    #[test]
    fn trailing_slash_of_home_is_not_doubled() {
        assert_eq!(expand_home_dir("~/a.yaml", Some("/root/")), "/root/a.yaml");
        assert_eq!(expand_home_dir("~/a.yaml", Some("/")), "/a.yaml");
    }

    #[test]
    fn tilde_inside_the_path_is_kept() {
        assert_eq!(
            expand_home_dir("/data/s3~viewer/settings.yaml", Some("/home/user")),
            "/data/s3~viewer/settings.yaml"
        );
        assert_eq!(
            expand_home_dir("~/s3~viewer/~settings.yaml", Some("/home/user")),
            "/home/user/s3~viewer/~settings.yaml"
        );
    }

    #[test]
    fn tilde_user_and_relative_paths_are_kept() {
        assert_eq!(
            expand_home_dir("~other/settings.yaml", Some("/home/user")),
            "~other/settings.yaml"
        );
        assert_eq!(
            expand_home_dir("settings.yaml", Some("/home/user")),
            "settings.yaml"
        );
    }

    #[test]
    fn unknown_home_keeps_the_path() {
        assert_eq!(expand_home_dir("~/.s3-viewer", None), "~/.s3-viewer");
    }
}
