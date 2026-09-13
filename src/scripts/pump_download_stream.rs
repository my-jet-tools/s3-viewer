use my_http_server::HttpOutputProducer;
use my_logger::LogEventCtx;
use my_s3::S3DownloadStream;

use crate::models::DownloadTarget;

// Moves the object from S3 to the HTTP response chunk by chunk, so memory stays at a few chunks
// whatever the object size. Runs detached from the handler, which has already returned the
// streamed response.
//
// - S3 fails mid-way: logged, and the pump stops. The response ends short; with Content-Length
//   set (S3 always sends it for GetObject) the client sees an incomplete download, not a
//   corrupted "complete" file.
// - The client goes away: `send` fails, the pump stops quietly - nobody is waiting any more.
pub async fn pump_download_stream(
    mut stream: S3DownloadStream,
    mut producer: HttpOutputProducer,
    target: DownloadTarget,
) {
    loop {
        let chunk = match stream.get_next_chunk().await {
            Ok(Some(chunk)) => chunk,
            Ok(None) => return,
            Err(err) => {
                my_logger::LOGGER.write_error(
                    "pump_download_stream",
                    format!("S3 stream failed mid-download, the response is truncated: {err}"),
                    LogEventCtx::new()
                        .add("bucket", target.bucket)
                        .add("key", target.key),
                );
                return;
            }
        };

        if producer.send(chunk).await.is_err() {
            return;
        }
    }
}
