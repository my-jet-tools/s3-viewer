/// `2026-09-13T14:05:32.000Z` -> `2026-09-13 14:05:32 UTC`.
///
/// S3 timestamps arrive as ISO-8601 in UTC and are passed through by the server as they are.
/// Anything that is not in that shape is shown exactly as it came, rather than guessed at.
pub fn format_last_modified(src: &str) -> String {
    let bytes = src.as_bytes();

    let looks_like_iso = bytes.len() >= 19
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes[10] == b'T'
        && bytes[13] == b':'
        && bytes[16] == b':'
        && src.is_char_boundary(19);

    if !looks_like_iso {
        return src.to_string();
    }

    let date = &src[..10];
    let time = &src[11..19];

    if src.ends_with('Z') || src.ends_with("+00:00") {
        format!("{date} {time} UTC")
    } else {
        format!("{date} {time}")
    }
}
