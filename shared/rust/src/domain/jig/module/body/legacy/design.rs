pub use super::*;
use crate::domain::jig::module::body::Transform;
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
    pub src: String,
    pub transform_matrix: [f64; 16],
    pub show_kind: ShowKind,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Text {
    pub html: String,
    pub width: f64,
    pub height: f64,
    pub transform_matrix: [f64; 16],
    pub show_kind: ShowKind,
}

#[repr(u8)]
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone, Copy)]
pub enum ShowKind {
    ShowOnLoad,
    HideOnTap,
    ShowOnTap,
}
