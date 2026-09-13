use super::ListedObject;

// A listed object together with the prefix it was listed under - the prefix is what its display
// name is relative to. Mapped to `FileHttpModel` in `mappers/level_file.rs`.
pub struct LevelFile<'s> {
    pub listed_prefix: &'s str,
    pub object: ListedObject,
}
