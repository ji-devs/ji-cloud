#![feature(trait_alias)]
#![feature(type_alias_impl_trait)]
#![feature(min_type_alias_impl_trait)]
//see: https://github.com/rust-lang/cargo/issues/8010
#![cfg_attr(feature = "quiet", allow(warnings))]

/// image itself is always enabled
/// but image_search is gated behind that
pub mod image;

#[cfg(feature = "module")]
pub mod module;
#[cfg(feature = "tooltip")]
pub mod tooltip;
#[cfg(feature = "color_select")]
pub mod color_select;
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
#[cfg(feature = "transform")]
pub mod transform;
#[cfg(feature = "stickers")]
pub mod stickers;
#[cfg(feature = "backgrounds")]
pub mod backgrounds;
#[cfg(feature = "traces")]
pub mod traces;
#[cfg(feature = "lists")]
pub mod lists;
#[cfg(feature = "audio_mixer")]
pub mod audio_mixer;
#[cfg(feature = "theme_selector")]
pub mod theme_selector;
#[cfg(feature = "collision")]
pub mod collision;

