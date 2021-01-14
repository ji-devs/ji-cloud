use serde::{Serialize, Deserialize};
use serde_repr::*;
pub use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Design {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bg: Option<String>,
    pub stickers: Vec<Sticker>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Sticker {
    Image(Image),
    Text(Text)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
    pub src: String,
    pub width: f64,
    pub height: f64,
    pub transform: [f64;6],
    pub show_kind: ShowKind, 
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Text {
    pub html: String,
    pub width: f64,
    pub height: f64,
    pub transform: [f64;6],
    pub show_kind: ShowKind, 
}
#[repr(u8)]
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone, Copy)]
pub enum ShowKind {
    ShowOnLoad,
    HideOnTap,
    ShowOnTap,
}

/*
#[derive(Serialize, Deserialize, Debug)]
pub struct Layer {
    pub width: f64,
    pub height: f64,
    pub transform: Transform,

    #[serde(rename="InteractiveLoopType")]
    pub play_kind: PlayKind,

    #[serde(rename="InteractiveShowType")]
    pub show_kind: ShowKind,

    #[serde(rename="interactiveLayerSound")]
    pub audio: String,

    #[serde(rename="interactiveToggleShow")]
    pub toggle_show: bool,

    #[serde(rename="type", deserialize_with = "layer_kind_deser")]
    pub layer_kind: LayerKind,

    pub filename: Option<String>,

    #[serde(rename="info")]
    pub html: Option<String>,
}


#[repr(u8)]
#[derive(Deserialize_repr, PartialEq, Debug, Clone, Copy)]
pub enum PlayKind {
    LoopOnLoad,
    LoopOnTap,
    OnceOnTap,
    OnceOnLoad,
}

#[repr(u8)]
#[derive(Deserialize_repr, PartialEq, Debug, Clone, Copy)]
pub enum ShowKind {
    ShowOnLoad,
    HideOnTap,
    ShowOnTap,
}

#[repr(u8)]
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum LayerKind {
    Background,
    Animation,
    Image,
    Text,
}
*/
