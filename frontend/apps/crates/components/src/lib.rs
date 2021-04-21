#![feature(trait_alias)]
#![feature(type_alias_impl_trait)]

//see: https://github.com/rust-lang/cargo/issues/8010
#![cfg_attr(feature = "quiet", allow(warnings))]

pub mod module;
pub mod tooltip;
pub mod image;
pub mod color_select;
pub mod image_search;
pub mod audio_input;
pub mod text_editor_controls;
pub mod font_loader;
