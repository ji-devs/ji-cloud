//see: https://github.com/rust-lang/cargo/issues/8010
#![cfg_attr(feature = "quiet", allow(warnings))]

pub mod prelude;

mod geom;
mod init;
mod render;
mod texture;
mod sprite;
mod primitive;
mod material;

