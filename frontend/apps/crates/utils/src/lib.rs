//see: https://github.com/rust-lang/cargo/issues/8010
#![cfg_attr(feature = "quiet", allow(warnings))]

pub mod ages;
pub mod api_helpers;
pub mod clipboard;
pub mod colors;
pub mod drag;
pub mod env;
pub mod events;
pub mod fetch;
pub mod firebase;
pub mod fonts;
pub mod iframe;
pub mod image;
pub mod image_effects;
pub mod init;
pub mod jig;
pub mod languages;
pub mod logging;
pub mod math;
pub mod mixin;
pub mod panic_hook;
pub mod path;
pub mod prelude;
pub mod resize;
pub mod routes;
pub mod screenshot;
pub mod storage;
pub mod themes;
pub mod unwrap;
