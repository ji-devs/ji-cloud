pub use super::*;
use serde::{Deserialize, Serialize};
use serde_repr::*;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Design {
    /// Background layer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bg: Option<String>,

    /// Stickers layer
    pub stickers: Vec<Sticker>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Sticker {
    /// images
    Image(Image),
    /// text
    Text(Text),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Image {
    pub src: String,
    pub width: f64,
    pub height: f64,
    pub transform: [f64; 6],
    pub show_kind: ShowKind,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Text {
    pub html: String,
    pub width: f64,
    pub height: f64,
    pub transform: [f64; 6],
    pub show_kind: ShowKind,
}

#[repr(u8)]
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone, Copy)]
pub enum ShowKind {
    ShowOnLoad,
    HideOnTap,
    ShowOnTap,
}
