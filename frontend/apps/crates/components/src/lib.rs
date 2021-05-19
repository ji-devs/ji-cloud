#![feature(trait_alias)]
#![feature(type_alias_impl_trait)]
#![feature(min_type_alias_impl_trait)]

//see: https://github.com/rust-lang/cargo/issues/8010
#![cfg_attr(feature = "quiet", allow(warnings))]

#[cfg(feature = "module")]
pub mod module;
#[cfg(feature = "tooltip")]
pub mod tooltip;
#[cfg(feature = "image")]
pub mod image;
#[cfg(feature = "color_select")]
pub mod color_select;
#[cfg(feature = "image_search")]
pub mod image_search;
#[cfg(feature = "audio_input")]
pub mod audio_input;
#[cfg(feature = "text_editor")]
pub mod text_editor;
#[cfg(feature = "font_loader")]
pub mod font_loader;
#[cfg(feature = "instructions")]
pub mod instructions;
#[cfg(feature = "animation")]
pub mod animation;
#[cfg(feature = "audio_player")]
pub mod audio_player;
#[cfg(feature = "transform")]
pub mod transform;
#[cfg(feature = "stickers")]
pub mod stickers;
#[cfg(feature = "background")]
pub mod backgrounds;
