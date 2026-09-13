use rest_api_shared::FolderHttpModel;

use crate::models::LevelFolder;

impl From<LevelFolder<'_>> for FolderHttpModel {
    fn from(folder: LevelFolder<'_>) -> Self {
        Self {
            name: crate::scripts::get_folder_name(folder.listed_prefix, &folder.common_prefix)
                .to_string(),
            prefix: folder.common_prefix,
        }
    }
}
