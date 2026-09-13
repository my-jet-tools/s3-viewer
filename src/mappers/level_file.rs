use rest_api_shared::FileHttpModel;

use crate::models::LevelFile;

impl From<LevelFile<'_>> for FileHttpModel {
    fn from(file: LevelFile<'_>) -> Self {
        Self {
            name: crate::scripts::get_file_name(file.listed_prefix, &file.object.key).to_string(),
            key: file.object.key,
            size: file.object.size,
            last_modified: file.object.last_modified,
        }
    }
}
