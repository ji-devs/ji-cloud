#![feature(trait_alias)]
#![feature(type_alias_impl_trait)]
//see: https://github.com/rust-lang/cargo/issues/8010
#![cfg_attr(feature = "quiet", allow(warnings))]

/// these are always enabled
pub mod input;

pub mod audio;
/// these are always enabled
/// with sub-modules gated
pub mod image;

#[cfg(feature = "animation")]
pub mod animation;
#[cfg(feature = "backgrounds")]
pub mod backgrounds;
#[cfg(feature = "box_outline")]
pub mod box_outline;
#[cfg(feature = "buttons")]
pub mod buttons;
#[cfg(feature = "collision")]
pub mod collision;
#[cfg(feature = "color_select")]
pub mod color_select;
#[cfg(feature = "hebrew_buttons")]
pub mod hebrew_buttons;
#[cfg(feature = "firebase")]
pub mod firebase;
#[cfg(feature = "instructions")]
pub mod instructions;
#[cfg(feature = "lists")]
pub mod lists;
#[cfg(feature = "module")]
pub mod module;
#[cfg(feature = "page_footer")]
pub mod page_footer;
#[cfg(feature = "page_header")]
pub mod page_header;
#[cfg(feature = "player_popup")]
pub mod player_popup;
#[cfg(feature = "share_jig")]
pub mod share_jig;
#[cfg(feature = "stickers")]
pub mod stickers;
#[cfg(feature = "tabs")]
pub mod tabs;
#[cfg(feature = "text_editor")]
pub mod text_editor;
#[cfg(feature = "theme_selector")]
pub mod theme_selector;
#[cfg(feature = "tooltip")]
pub mod tooltip;
#[cfg(feature = "traces")]
pub mod traces;
#[cfg(feature = "transform")]
pub mod transform;
#[cfg(feature = "overlay")]
pub mod overlay;
