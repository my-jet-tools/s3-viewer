pub const DEFAULT_CONTENT_TYPE: &str = "application/octet-stream";

// The `Content-Type` to answer a download with: the one S3 stored for the object when it is a
// well-formed header value, `application/octet-stream` otherwise. The stored value is whatever the
// uploader sent, so anything outside visible ASCII is not trusted into a response header.
pub fn resolve_content_type(s3_content_type: Option<&str>) -> &str {
    let Some(content_type) = s3_content_type else {
        return DEFAULT_CONTENT_TYPE;
    };

    let content_type = content_type.trim();

    let is_visible_ascii = content_type
        .bytes()
        .all(|byte| byte == b'\t' || (0x20..=0x7E).contains(&byte));

    if content_type.is_empty() || !is_visible_ascii {
        return DEFAULT_CONTENT_TYPE;
    }

    content_type
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn s3_value_is_used() {
        assert_eq!(resolve_content_type(Some("image/png")), "image/png");
        assert_eq!(
            resolve_content_type(Some(" text/plain; charset=utf-8 ")),
            "text/plain; charset=utf-8"
        );
    }

    #[test]
    fn missing_or_empty_falls_back() {
        assert_eq!(resolve_content_type(None), DEFAULT_CONTENT_TYPE);
        assert_eq!(resolve_content_type(Some("  ")), DEFAULT_CONTENT_TYPE);
    }

    #[test]
    fn malformed_value_falls_back() {
        assert_eq!(
            resolve_content_type(Some("text/html\r\nSet-Cookie: x=1")),
            DEFAULT_CONTENT_TYPE
        );
        assert_eq!(
            resolve_content_type(Some("текст/plain")),
            DEFAULT_CONTENT_TYPE
        );
    }
}
