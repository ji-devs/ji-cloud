use serde::{de, Deserializer,Deserialize};
use serde_repr::*;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::fmt;
use super::*;

#[derive(Deserialize, Debug)]
pub struct Layer {
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub transform: Option<Transform>,

    #[serde(rename="InteractiveLoopType")]
    pub loop_kind: Option<LoopKind>,

    #[serde(rename="InteractiveShowType")]
    pub show_kind: Option<ShowKind>,

    #[serde(rename="interactiveLayerSound")]
    pub audio: Option<String>,

    #[serde(rename="interactiveToggleShow")]
    pub toggle_show: Option<bool>,

    #[serde(rename="type", deserialize_with = "layer_kind_deser")]
    pub kind: LayerKind,

    pub filename: Option<String>,

    #[serde(rename="info")]
    pub html: Option<String>,
}


#[repr(u8)]
#[derive(Deserialize_repr, PartialEq, Debug, Clone, Copy)]
pub enum LoopKind {
    PlayOnLoadLoop,
    PlayOnTapLoop,
    PlayOnTapOnce,
    PlayOnLoadOnce,
}

impl Default for LoopKind {
    fn default() -> Self {
        Self::PlayOnLoadLoop
    }
}

#[repr(u8)]
#[derive(Deserialize_repr, PartialEq, Debug, Clone, Copy)]
pub enum ShowKind {
    ShowOnLoad,
    HideOnTap,
    ShowOnTap,
}

impl Default for ShowKind {
    fn default() -> Self {
        Self::ShowOnLoad
    }
}

#[repr(u8)]
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum LayerKind {
    Background,
    Animation,
    Image,
    Text,
}
pub fn layer_kind_deser<'de, D>(deserializer: D) -> Result<LayerKind, D::Error>
where
    D: Deserializer<'de>,
{
    let s:String = Deserialize::deserialize(deserializer)?;

    match s.as_ref() {
        "bg" => Ok(LayerKind::Background),
        "anim" => Ok(LayerKind::Animation),
        "img" => Ok(LayerKind::Image),
        "txt" => Ok(LayerKind::Text),
        _ => Err(serde::de::Error::custom(format!("unknown layer type [{}]!", s)))
    }

}
