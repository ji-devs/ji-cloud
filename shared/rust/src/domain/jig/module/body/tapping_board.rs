use crate::domain::jig::module::{
    body::{Audio, Backgrounds, Body, BodyExt, Instructions, Sticker, ThemeChoice, Trace},
    ModuleKind,
};
#[cfg(feature = "backend")]
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

mod play_settings;
pub use play_settings::*;

/// The body for [`TappingBoard`](crate::domain::jig::module::ModuleKind::TappingBoard) modules.
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct ModuleData {
    /// The content
    pub content: Option<Content>,
}

impl BodyExt<Mode> for ModuleData {
    fn as_body(&self) -> Body {
        Body::TappingBoard(self.clone())
    }

    fn is_complete(&self) -> bool {
        self.content.is_some()
    }

    fn kind() -> ModuleKind {
        ModuleKind::TappingBoard
    }

    fn new_mode(mode: Mode) -> Self {
        ModuleData {
            content: Some(Content::default())
        }
    }
}

impl TryFrom<Body> for ModuleData {
    type Error = &'static str;

    fn try_from(body: Body) -> Result<Self, Self::Error> {
        match body {
            Body::TappingBoard(data) => Ok(data),
            _ => Err("cannot convert body to tapping board!"),
        }
    }
}

/// The body for [`TappingBoard`](crate::domain::jig::module::ModuleKind::TappingBoard) modules.
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct Content {
    /// The mode
    pub mode: Mode,

    /// The instructions for the module.
    pub instructions: Instructions,

    /// The module's theme.
    pub theme: ThemeChoice,

    /// Backgrounds
    pub backgrounds: Backgrounds,

    /// Stickers
    pub stickers: Vec<Sticker>,

    /// Traces
    pub traces: Vec<TappingTrace>,

    /// play settings
    pub play_settings: PlaySettings,
}

/// Tapping board trace w/ metadata
#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct TappingTrace {
    /// the trace
    pub trace: Trace,

    /// audio
    pub audio: Option<Audio>,

    /// text
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
/// The mode
pub enum Mode {
    /// Printables
    Printables,
    /// TalkingPictures
    TalkingPictures,
    /// Comics
    Comics,
    /// Timeline
    Timeline,
    /// Family Tree
    FamilyTree,
    /// Poster
    Poster,
}

impl Default for Mode {
    fn default() -> Self {
        Self::Poster
    }
}
