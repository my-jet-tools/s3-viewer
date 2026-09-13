const BASE_INDENT_PX: usize = 6;
const INDENT_STEP_PX: usize = 16;

/// Left padding of a tree row at `depth` (buckets are depth 0).
pub fn row_indent(depth: usize) -> usize {
    BASE_INDENT_PX + depth * INDENT_STEP_PX
}
