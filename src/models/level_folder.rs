// A common prefix S3 returned together with the prefix it was listed under - the prefix is what
// its display name is relative to. Mapped to `FolderHttpModel` in `mappers/level_folder.rs`.
pub struct LevelFolder<'s> {
    pub listed_prefix: &'s str,
    pub common_prefix: String,
}
