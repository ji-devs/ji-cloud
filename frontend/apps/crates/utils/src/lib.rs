//see: https://github.com/rust-lang/cargo/issues/8010
#![cfg_attr(feature = "quiet", allow(warnings))]

pub mod fetch;
pub mod storage;
pub mod routes;
pub mod settings;
pub mod path;
pub mod firebase;
pub mod iframe;
pub mod resize;
pub mod math;
pub mod drag;
pub mod events;
pub mod api_helpers;
pub mod prelude;
pub mod unwrap;
pub mod themes;
pub mod colors;

pub(crate) mod strings;
