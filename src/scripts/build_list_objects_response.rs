use rest_api_shared::{FileHttpModel, FolderHttpModel, ListObjectsResponse};

use crate::models::{LevelFile, LevelFolder, ListedLevel};

// Turns a raw listing into the response: the folder marker skipped, every entry mapped with the
// prefix its display name is relative to (see `mappers/`), folders and files each sorted by name
// (case-insensitive, ties broken by the exact name so the order is stable).
pub fn build_list_objects_response(
    bucket: &str,
    prefix: String,
    level: ListedLevel,
) -> ListObjectsResponse {
    let mut folders: Vec<FolderHttpModel> = level
        .common_prefixes
        .into_iter()
        .map(|common_prefix| {
            FolderHttpModel::from(LevelFolder {
                listed_prefix: prefix.as_str(),
                common_prefix,
            })
        })
        .collect();

    let mut files: Vec<FileHttpModel> = level
        .objects
        .into_iter()
        .filter(|object| !super::is_folder_marker(&prefix, &object.key, object.size))
        .map(|object| {
            FileHttpModel::from(LevelFile {
                listed_prefix: prefix.as_str(),
                object,
            })
        })
        .collect();

    folders.sort_by_cached_key(|folder| (folder.name.to_lowercase(), folder.name.clone()));
    files.sort_by_cached_key(|file| (file.name.to_lowercase(), file.name.clone()));

    ListObjectsResponse {
        bucket: bucket.to_string(),
        prefix,
        folders,
        files,
        is_truncated: level.is_truncated,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::ListedObject;

    fn object(key: &str, size: u64) -> ListedObject {
        ListedObject {
            key: key.to_string(),
            size,
            last_modified: "2026-09-13T10:00:00.000Z".to_string(),
        }
    }

    fn level(common_prefixes: &[&str], objects: Vec<ListedObject>) -> ListedLevel {
        ListedLevel {
            common_prefixes: common_prefixes.iter().map(ToString::to_string).collect(),
            objects,
            is_truncated: false,
        }
    }

    #[test]
    fn bucket_root() {
        let response = build_list_objects_response(
            "b1",
            String::new(),
            level(&["photos/", "docs/"], vec![object("readme.txt", 5)]),
        );

        assert_eq!(response.bucket, "b1");
        assert_eq!(response.prefix, "");
        assert_eq!(
            response.folders,
            vec![
                FolderHttpModel {
                    prefix: "docs/".to_string(),
                    name: "docs".to_string()
                },
                FolderHttpModel {
                    prefix: "photos/".to_string(),
                    name: "photos".to_string()
                },
            ]
        );
        assert_eq!(response.files.len(), 1);
        assert_eq!(response.files[0].key, "readme.txt");
        assert_eq!(response.files[0].name, "readme.txt");
        assert_eq!(response.files[0].size, 5);
        assert!(!response.is_truncated);
    }

    #[test]
    fn nested_level_skips_the_folder_marker() {
        let response = build_list_objects_response(
            "b1",
            "a/b/".to_string(),
            level(
                &["a/b/c/"],
                vec![object("a/b/", 0), object("a/b/file 1.txt", 12)],
            ),
        );

        assert_eq!(response.folders[0].prefix, "a/b/c/");
        assert_eq!(response.folders[0].name, "c");
        assert_eq!(response.files.len(), 1);
        assert_eq!(response.files[0].key, "a/b/file 1.txt");
        assert_eq!(response.files[0].name, "file 1.txt");
    }

    #[test]
    fn sorted_by_name_case_insensitive_with_unicode() {
        let response = build_list_objects_response(
            "b1",
            "x/".to_string(),
            level(
                &["x/beta/", "x/Alpha/", "x/яблоко/", "x/alpha/"],
                vec![
                    object("x/b.txt", 1),
                    object("x/A.txt", 1),
                    object("x/ä.txt", 1),
                ],
            ),
        );

        let folder_names: Vec<&str> = response.folders.iter().map(|f| f.name.as_str()).collect();
        assert_eq!(folder_names, vec!["Alpha", "alpha", "beta", "яблоко"]);

        let file_names: Vec<&str> = response.files.iter().map(|f| f.name.as_str()).collect();
        assert_eq!(file_names, vec!["A.txt", "b.txt", "ä.txt"]);
    }

    #[test]
    fn truncation_is_passed_through() {
        let mut listed = level(&[], vec![]);
        listed.is_truncated = true;

        let response = build_list_objects_response("b1", String::new(), listed);

        assert!(response.is_truncated);
        assert!(response.folders.is_empty());
        assert!(response.files.is_empty());
    }
}
