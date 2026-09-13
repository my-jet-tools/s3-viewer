use my_http_server::{HttpOutput, HttpOutputAsStream};
use rest_api_shared::DownloadObjectInputModel;

use crate::app::AppContext;
use crate::models::{DownloadTarget, S3ViewerError};

// How many chunks may wait for a slow client. Bounded on purpose: back pressure reaches S3, and a
// multi-gigabyte object costs a few chunks of memory.
const DOWNLOAD_CHUNKS_IN_FLIGHT: usize = 4;

// Opens the object and starts pumping it into a streamed response, which is returned with its
// headers set. The action turns it into the HTTP result.
pub async fn download_object(
    app: &AppContext,
    input: DownloadObjectInputModel,
) -> Result<HttpOutputAsStream, S3ViewerError> {
    let stream = crate::scripts::open_download_stream(
        &app.s3_buckets,
        input.bucket.as_str(),
        input.key.as_str(),
    )
    .await?;

    let content_type =
        crate::scripts::resolve_content_type(stream.content_type.as_deref()).to_string();
    let content_length = stream.content_length;

    let (output, producer) = HttpOutput::as_stream(DOWNLOAD_CHUNKS_IN_FLIGHT);

    let mut output = output
        .with_header("Content-Type", content_type)
        .with_header(
            "Content-Disposition",
            crate::scripts::build_content_disposition(input.key.as_str()),
        )
        // The stored Content-Type is whatever the uploader chose; never let a browser sniff it
        // into something executable.
        .with_header("X-Content-Type-Options", "nosniff");

    if let Some(content_length) = content_length {
        output = output.with_header("Content-Length", content_length.to_string());
    }

    tokio::spawn(crate::scripts::pump_download_stream(
        stream,
        producer,
        DownloadTarget {
            bucket: input.bucket,
            key: input.key,
        },
    ));

    Ok(output)
}
