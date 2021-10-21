use std::time::Duration;

pub use super::*;
use serde::{Deserialize, Serialize};
use serde_repr::*;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Design {
    /// Background layer
    pub bgs: Vec<String>,

    /// Stickers layer
    pub stickers: Vec<Sticker>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Sticker {
    /// sprites
    Sprite(Sprite),
    /// text
    Text(Text),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sprite {
    pub src: SpriteSource,
    pub transform_matrix: [f64; 16],
    /// start out hidden
    pub hide: bool,
    /// toggle hidden state
    pub hide_toggle: Option<HideToggle>,
    /// animation options
    pub animation: Option<Animation>,
    // associated audio
    pub audio: Option<String>,
}

// in the case of full image, it's the filename
/// for video, it's without the extension, so client can load .webm, .mp4, etc.
pub type SpriteSource = String;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Text {
    pub html: String,
    pub width: f64,
    pub height: f64,
    pub transform_matrix: [f64; 16],
    /// start out hidden
    pub hide: bool,
    /// toggle hidden state
    pub hide_toggle: Option<HideToggle>,
    // associated audio
    pub audio: Option<String>,
}

#[repr(u8)]
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone, Copy)]
pub enum HideToggle {
    /// only let the toggle fire once
    Once,
    /// let the toggle fire indefinitely
    Always,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Animation {
    /// do not let the animation loop
    pub once: bool,
    /// wait for tap before playing
    pub tap: bool,
}
