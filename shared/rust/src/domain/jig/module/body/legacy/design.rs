use crate::domain::{audio::AudioId, image::ImageId};

pub use super::*;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Design {
    /// Background layer
    pub bgs: Vec<String>,

    /// Stickers layer
    pub stickers: Vec<Sticker>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Sticker {
    /// sprites
    Sprite(Sprite),
    /// text
    Text(Text),
}

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
    pub audio_filename: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sprite {
    pub filename: String,
    pub transform_matrix: [f64; 16],
    /// hide and hide_toggle are mapped from the top sections
    /// in "Houdini": HideOnTap, ShowOnTap, and ToggleOnTap
    /// start out hidden
    pub hide: bool,
    /// toggle hidden state
    pub hide_toggle: Option<HideToggle>,

    /// animation options are mapped from the bottom animation section
    pub animation: Option<Animation>,
    // associated audio
    pub audio_filename: Option<String>,

    /// override the size
    pub override_size: Option<(f64, f64)>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
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
