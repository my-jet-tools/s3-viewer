use std::sync::Arc;

use rust_extensions::AppStates;

use crate::s3_buckets::S3Buckets;
use crate::settings::SettingsModel;

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");

pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct AppContext {
    // One S3 client per configured bucket. Immutable after start, so no locks.
    pub s3_buckets: S3Buckets,

    // Initialized from the start: nothing has to be loaded before the first request.
    pub states: Arc<AppStates>,

    pub settings: SettingsModel,
}

impl AppContext {
    pub fn new(settings: SettingsModel) -> Self {
        Self {
            s3_buckets: S3Buckets::new(&settings.buckets),
            states: Arc::new(AppStates::create_initialized()),
            settings,
        }
    }
}
