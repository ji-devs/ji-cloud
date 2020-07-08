//see: https://github.com/rust-lang/cargo/issues/8010
#![cfg_attr(feature = "quiet", allow(warnings))]

extern crate openssl;

pub mod settings;
pub mod user;
pub mod auth;
