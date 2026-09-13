use std::fmt;

/// The error every API call returns: a transport failure, a non-2xx answer (with the server's
/// message) or a body that does not deserialize.
#[derive(Debug)]
pub struct RequestError {
    pub message: String,
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<flurl::FlUrlError> for RequestError {
    fn from(err: flurl::FlUrlError) -> Self {
        Self {
            message: err.to_string(),
        }
    }
}

// FlUrl's JSON helpers can surface serde_json errors, so the helpers in api/ may use `?` on them.
impl From<serde_json::Error> for RequestError {
    fn from(err: serde_json::Error) -> Self {
        Self {
            message: err.to_string(),
        }
    }
}
