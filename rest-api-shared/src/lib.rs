//! Wire models of the s3-viewer REST API - pure data, no behaviour.
//!
//! Request models derive `MyHttpInput`, response models derive `MyHttpObjectStructure`.
//! The route constants are what the UI calls; the server's `#[http_route]` literals must match them.
//!
//! Never put `///` doc comments on fields of these structs - the derive macros panic on them.

mod buckets;
pub use buckets::*;
mod objects;
pub use objects::*;
