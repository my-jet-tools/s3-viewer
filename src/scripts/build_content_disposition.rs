// Used when the key yields no usable file name at all.
const FALLBACK_FILE_NAME: &str = "download";

const HEX_DIGITS: &[u8; 16] = b"0123456789ABCDEF";

// `Content-Disposition` for downloading `key` as an attachment named after its last segment:
//
//   attachment; filename="<ASCII fallback>"; filename*=UTF-8''<RFC 5987 percent-encoded name>
//
// The key is user data (anybody who can upload can pick it), so nothing of it reaches the header
// unescaped: the fallback keeps printable ASCII minus quotes, backslashes and '%', and the encoded
// form only ever contains RFC 5987 attr-chars and %XX. No CR/LF, no quote can end the parameter
// early.
pub fn build_content_disposition(key: &str) -> String {
    let file_name = get_download_file_name(key);

    format!(
        "attachment; filename=\"{}\"; filename*=UTF-8''{}",
        to_ascii_fallback(file_name),
        percent_encode_rfc5987(file_name)
    )
}

fn get_download_file_name(key: &str) -> &str {
    let trimmed = key.trim_end_matches('/');
    let name = trimmed.rsplit('/').next().unwrap_or_default();

    if name.is_empty() {
        FALLBACK_FILE_NAME
    } else {
        name
    }
}

// For clients that do not understand `filename*`: non-ASCII becomes '_', quotes, backslashes and
// control characters are dropped. '%' becomes '_' too: RFC 6266 (4.3, appendix D) warns that some
// user agents percent-decode `filename`, which would turn "100%41.txt" into "100A.txt".
fn to_ascii_fallback(file_name: &str) -> String {
    let mut result = String::with_capacity(file_name.len());

    for c in file_name.chars() {
        if c == '"' || c == '\\' || c.is_control() {
            continue;
        }

        if c.is_ascii() && c != '%' {
            result.push(c);
        } else {
            result.push('_');
        }
    }

    if result.trim().is_empty() {
        return FALLBACK_FILE_NAME.to_string();
    }

    result
}

fn percent_encode_rfc5987(value: &str) -> String {
    let mut result = String::with_capacity(value.len() * 3);

    for byte in value.bytes() {
        if is_rfc5987_attr_char(byte) {
            result.push(byte as char);
        } else {
            result.push('%');
            result.push(HEX_DIGITS[(byte >> 4) as usize] as char);
            result.push(HEX_DIGITS[(byte & 0x0F) as usize] as char);
        }
    }

    result
}

// attr-char = ALPHA / DIGIT / "!" / "#" / "$" / "&" / "+" / "-" / "." / "^" / "_" / "`" / "|" / "~"
fn is_rfc5987_attr_char(byte: u8) -> bool {
    byte.is_ascii_alphanumeric()
        || matches!(
            byte,
            b'!' | b'#' | b'$' | b'&' | b'+' | b'-' | b'.' | b'^' | b'_' | b'`' | b'|' | b'~'
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_is_valid_header_value(value: &str) {
        assert!(!value.contains('\r'));
        assert!(!value.contains('\n'));
        assert!(
            my_http_server::hyper::header::HeaderValue::from_str(value).is_ok(),
            "not a valid header value: {value:?}"
        );
    }

    #[test]
    fn plain_ascii_name_from_a_nested_key() {
        let value = build_content_disposition("a/b/report.pdf");

        assert_eq!(
            value,
            "attachment; filename=\"report.pdf\"; filename*=UTF-8''report.pdf"
        );
        assert_is_valid_header_value(&value);
    }

    #[test]
    fn spaces_are_kept_in_fallback_and_encoded_in_filename_star() {
        let value = build_content_disposition("docs/my file (1).txt");

        assert_eq!(
            value,
            "attachment; filename=\"my file (1).txt\"; filename*=UTF-8''my%20file%20%281%29.txt"
        );
        assert_is_valid_header_value(&value);
    }

    #[test]
    fn unicode_name() {
        let value = build_content_disposition("фото/лето.jpg");

        assert_eq!(
            value,
            "attachment; filename=\"____.jpg\"; filename*=UTF-8''%D0%BB%D0%B5%D1%82%D0%BE.jpg"
        );
        assert_is_valid_header_value(&value);
    }

    #[test]
    fn quotes_backslashes_and_line_breaks_can_not_inject() {
        let value = build_content_disposition("x/a\"b\\c\r\nSet-Cookie: x=1;.txt");

        assert_eq!(
            value,
            "attachment; filename=\"abcSet-Cookie: x=1;.txt\"; filename*=UTF-8''a%22b%5Cc%0D%0ASet-Cookie%3A%20x%3D1%3B.txt"
        );
        assert_is_valid_header_value(&value);
    }

    #[test]
    fn percent_is_not_left_for_the_fallback_to_be_decoded() {
        let value = build_content_disposition("x/100%41.txt");

        assert_eq!(
            value,
            "attachment; filename=\"100_41.txt\"; filename*=UTF-8''100%2541.txt"
        );
        assert_is_valid_header_value(&value);
    }

    #[test]
    fn keys_without_a_usable_name_fall_back() {
        assert_eq!(
            build_content_disposition(""),
            "attachment; filename=\"download\"; filename*=UTF-8''download"
        );
        assert_eq!(
            build_content_disposition("a/b/"),
            "attachment; filename=\"b\"; filename*=UTF-8''b"
        );
        assert_eq!(
            build_content_disposition("\r\n"),
            "attachment; filename=\"download\"; filename*=UTF-8''%0D%0A"
        );
    }
}
