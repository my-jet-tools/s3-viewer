use super::S3ConnStringError;

// Longest unknown key name that is still echoed back in an error. Real key names are short words;
// anything longer, or not made of ASCII letters, may be a piece of a secret.
const MAX_PRINTABLE_KEY_LEN: usize = 16;

// One bucket of the tree, configured as a connection string in the same `Key=Value;Key=Value`
// shape my-service-bus-persistence uses for its `s3_conn_string`:
//
//   Endpoint=https://fsn1.your-objectstorage.com;Region=fsn1;AccessKey=...;SecretKey=...;Bucket=my-bucket
//
// Keys are case-sensitive. Only the first `=` of an entry separates, so a base64 secret keeps its
// own `=`; a value can not contain `;`. `Debug=1` traces every S3 request of that bucket to the
// console. No `Debug` derive on purpose: the struct carries credentials.
pub struct S3ConnString {
    pub endpoint: String,
    pub region: String,
    pub access_key: String,
    pub secret_key: String,
    pub bucket: String,
    pub debug: bool,
}

impl S3ConnString {
    pub fn parse(conn_string: &str) -> Result<Self, S3ConnStringError> {
        let mut endpoint = None;
        let mut region = None;
        let mut access_key = None;
        let mut secret_key = None;
        let mut bucket = None;
        let mut debug = None;

        for (index, entry) in conn_string.split(';').enumerate() {
            let entry = entry.trim();

            if entry.is_empty() {
                continue;
            }

            let entry_number = index + 1;

            let Some((key, value)) = entry.split_once('=') else {
                return Err(S3ConnStringError::NotKeyValue {
                    entry: entry_number,
                });
            };

            let key = key.trim();
            let value = value.trim();

            match key {
                "Endpoint" => set_once(&mut endpoint, "Endpoint", value)?,
                "Region" => set_once(&mut region, "Region", value)?,
                "AccessKey" => set_once(&mut access_key, "AccessKey", value)?,
                "SecretKey" => set_once(&mut secret_key, "SecretKey", value)?,
                "Bucket" => set_once(&mut bucket, "Bucket", value)?,
                "Debug" => {
                    if debug.is_some() {
                        return Err(S3ConnStringError::DuplicateKey { key: "Debug" });
                    }

                    debug = Some(parse_bool(value)?);
                }
                _ => {
                    return Err(S3ConnStringError::UnknownKey {
                        entry: entry_number,
                        key: printable_key(key),
                    });
                }
            }
        }

        Ok(Self {
            endpoint: required(endpoint, "Endpoint")?,
            region: required(region, "Region")?,
            access_key: required(access_key, "AccessKey")?,
            secret_key: required(secret_key, "SecretKey")?,
            bucket: required(bucket, "Bucket")?,
            debug: debug.unwrap_or(false),
        })
    }
}

fn set_once(
    slot: &mut Option<String>,
    key: &'static str,
    value: &str,
) -> Result<(), S3ConnStringError> {
    if slot.is_some() {
        return Err(S3ConnStringError::DuplicateKey { key });
    }

    *slot = Some(value.to_string());

    Ok(())
}

fn required(value: Option<String>, key: &'static str) -> Result<String, S3ConnStringError> {
    value
        .filter(|value| !value.is_empty())
        .ok_or(S3ConnStringError::MissingKey { key })
}

// Spelled out rather than `== "1"`: the value is typed by hand, and a `Debug=true` that silently
// means "off" is worse than a refusal to start.
fn parse_bool(value: &str) -> Result<bool, S3ConnStringError> {
    match value.to_ascii_lowercase().as_str() {
        "1" | "true" | "yes" | "on" => Ok(true),
        "0" | "false" | "no" | "off" => Ok(false),
        _ => Err(S3ConnStringError::InvalidDebugValue {
            value: value.to_string(),
        }),
    }
}

fn printable_key(key: &str) -> Option<String> {
    let looks_like_a_key = !key.is_empty()
        && key.len() <= MAX_PRINTABLE_KEY_LEN
        && key.chars().all(|c| c.is_ascii_alphabetic());

    looks_like_a_key.then(|| key.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID: &str = "Endpoint=https://s3.eu-central-1.amazonaws.com;Region=eu-central-1;AccessKey=AKIA123;SecretKey=abc/def+ghi=;Bucket=my-bucket";

    fn parse_ok(conn_string: &str) -> S3ConnString {
        match S3ConnString::parse(conn_string) {
            Ok(parsed) => parsed,
            Err(err) => panic!("'{conn_string}' should parse, got: {err}"),
        }
    }

    fn parse_err(conn_string: &str) -> S3ConnStringError {
        match S3ConnString::parse(conn_string) {
            Ok(_) => panic!("'{conn_string}' should be refused"),
            Err(err) => err,
        }
    }

    #[test]
    fn parses_a_connection_string() {
        let parsed = parse_ok(VALID);

        assert_eq!(parsed.endpoint, "https://s3.eu-central-1.amazonaws.com");
        assert_eq!(parsed.region, "eu-central-1");
        assert_eq!(parsed.access_key, "AKIA123");
        // A base64 secret carries its own '=' - only the first one separates.
        assert_eq!(parsed.secret_key, "abc/def+ghi=");
        assert_eq!(parsed.bucket, "my-bucket");
        assert!(!parsed.debug);
    }

    #[test]
    fn whitespace_and_empty_entries_are_tolerated() {
        let parsed =
            parse_ok(" Endpoint = https://s3 ; Region=eu;;AccessKey=a ; SecretKey=b;Bucket=c; ");

        assert_eq!(parsed.endpoint, "https://s3");
        assert_eq!(parsed.access_key, "a");
        assert_eq!(parsed.bucket, "c");
    }

    #[test]
    fn every_required_key_is_required() {
        let entries = [
            ("Endpoint", "Endpoint=https://s3"),
            ("Region", "Region=eu"),
            ("AccessKey", "AccessKey=a"),
            ("SecretKey", "SecretKey=b"),
            ("Bucket", "Bucket=c"),
        ];

        for (missing, _) in entries {
            let conn_string: Vec<&str> = entries
                .iter()
                .filter(|(key, _)| *key != missing)
                .map(|(_, entry)| *entry)
                .collect();

            assert_eq!(
                parse_err(conn_string.join(";").as_str()),
                S3ConnStringError::MissingKey { key: missing }
            );
        }
    }

    #[test]
    fn an_empty_value_is_missing() {
        assert_eq!(
            parse_err("Endpoint=https://s3;Region=eu;AccessKey=a;SecretKey=b;Bucket="),
            S3ConnStringError::MissingKey { key: "Bucket" }
        );
    }

    #[test]
    fn a_typo_is_an_unknown_key() {
        let err = parse_err("Endpoint=https://s3;Region=eu;AccessKey=a;SecretKey=b;Buckett=c");

        assert_eq!(
            err,
            S3ConnStringError::UnknownKey {
                entry: 5,
                key: Some("Buckett".to_string())
            }
        );
        assert!(err.to_string().contains("'Buckett'"));
    }

    #[test]
    fn keys_are_case_sensitive() {
        assert_eq!(
            parse_err("Endpoint=https://s3;Region=eu;AccessKey=a;SecretKey=b;bucket=c"),
            S3ConnStringError::UnknownKey {
                entry: 5,
                key: Some("bucket".to_string())
            }
        );
    }

    #[test]
    fn a_repeated_key_is_refused() {
        assert_eq!(
            parse_err("Endpoint=https://s3;Region=eu;Region=us;AccessKey=a;SecretKey=b;Bucket=c"),
            S3ConnStringError::DuplicateKey { key: "Region" }
        );
    }

    #[test]
    fn an_entry_without_equals_is_refused_without_printing_it() {
        let err = parse_err("Endpoint=https://s3;Region=eu;AccessKey=a;TopSecretValue;Bucket=c");

        assert_eq!(err, S3ConnStringError::NotKeyValue { entry: 4 });
        assert!(!err.to_string().contains("TopSecretValue"));
    }

    #[test]
    fn a_fragment_of_a_secret_is_not_printed() {
        // A secret that contains ';' splits into an entry whose "key" is part of the secret.
        let err =
            parse_err("Endpoint=https://s3;Region=eu;AccessKey=a;SecretKey=ab;c/d+e=f;Bucket=c");

        assert_eq!(
            err,
            S3ConnStringError::UnknownKey {
                entry: 5,
                key: None
            }
        );
        assert!(!err.to_string().contains("c/d+e"));
    }

    #[test]
    fn debug_is_switched_by_the_connection_string() {
        for value in ["1", "true", "TRUE", "yes", "on"] {
            assert!(
                parse_ok(format!("{VALID};Debug={value}").as_str()).debug,
                "'{value}' should switch debug on"
            );
        }

        for value in ["0", "false", "no", "off"] {
            assert!(
                !parse_ok(format!("{VALID};Debug={value}").as_str()).debug,
                "'{value}' should leave debug off"
            );
        }
    }

    #[test]
    fn an_unreadable_debug_value_is_refused() {
        assert_eq!(
            parse_err(format!("{VALID};Debug=maybe").as_str()),
            S3ConnStringError::InvalidDebugValue {
                value: "maybe".to_string()
            }
        );
    }
}
