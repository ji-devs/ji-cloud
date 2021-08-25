//see: https://github.com/rust-lang/cargo/issues/8010
#![cfg_attr(feature = "quiet", allow(warnings))]
#![feature(type_alias_impl_trait)]

pub mod image;
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
pub mod image_effects;
pub mod ages;
pub mod clipboard;
pub mod env;
pub mod fonts;
pub mod screenshot;
pub mod languages;
pub mod jig;
pub mod mixin;

pub(crate) mod strings;
