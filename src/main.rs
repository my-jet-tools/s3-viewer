// Lint levels live in Cargo.toml ([lints]), not here: a crate-level `#![deny(...)]` would override
// the manifest and re-deny the one framework-dictated exception declared there.

use std::sync::Arc;

mod app;
mod flows;
mod http_server;
mod mappers;
mod models;
mod s3_buckets;
mod scripts;
mod settings;

#[tokio::main]
async fn main() {
    let settings = crate::settings::read_settings().await;

    let app = Arc::new(crate::app::AppContext::new(settings));

    crate::http_server::start_http_server(&app);

    // Registers SIGINT / SIGTERM and returns once one arrives. There is nothing to flush on the
    // way out: the viewer is read-only and keeps no state of its own.
    app.states.wait_until_shutdown().await;

    println!("{} is shut down", crate::app::APP_NAME);
}
