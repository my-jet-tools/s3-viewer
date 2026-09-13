use flurl::{FlUrlError, FlUrlResponse};
use serde::de::DeserializeOwned;

use crate::models::RequestError;

/// The one place a FlUrl result becomes a typed result. Pass the raw
/// `FlUrl::new(...).execute_request(...).await` straight in: a transport error becomes a
/// `RequestError`, a non-2xx status becomes a `RequestError` carrying the response body, and a
/// 2xx response is deserialized into `T`.
pub async fn handle_http_response<T: DeserializeOwned>(
    response: Result<FlUrlResponse, FlUrlError>,
) -> Result<T, RequestError> {
    let mut response = response?;
    let status = response.get_status_code();

    if !is_success(status) {
        return Err(read_error_body(&mut response, status).await);
    }

    Ok(response.get_json().await?)
}

fn is_success(status: u16) -> bool {
    (200..300).contains(&status)
}

/// The server puts the reason into the body - surface it verbatim when there is one.
async fn read_error_body(response: &mut FlUrlResponse, status: u16) -> RequestError {
    let message = match response.get_body_as_str().await {
        Ok(body) if !body.trim().is_empty() => format!("HTTP {status}: {}", body.trim()),
        Ok(_) => format!("HTTP {status}"),
        Err(err) => format!("HTTP {status}: {err}"),
    };

    RequestError { message }
}
