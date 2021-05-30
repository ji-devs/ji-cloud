use crate::domain::jig::module::{ModuleKind, body::{BodyExt, Audio, Body, Instructions, Sticker, Trace, Backgrounds, ThemeId}};
use std::convert::TryFrom;
#[cfg(feature = "backend")]
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

/// The body for [`TappingBoard`](crate::domain::jig::module::ModuleKind::TappingBoard) modules.
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct ModuleData {
    /// The content
    pub content: Option<Content>,
}

impl BodyExt for ModuleData {
    fn as_body(&self) -> Body {
        Body::TappingBoard(self.clone())
    }

    fn is_complete(&self) -> bool {
        self.content.is_some()
    }

    fn kind() -> ModuleKind {
        ModuleKind::TappingBoard
    }
}

impl TryFrom<Body> for ModuleData {
    type Error = &'static str;

    fn try_from(body:Body) -> Result<Self, Self::Error> {
        match body {
            Body::TappingBoard(data) => Ok(data),
            _ => Err("cannot convert body to tapping board!")
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

    /// The ID of the module's theme.
    pub theme_id: ThemeId,

    /// Backgrounds
    pub backgrounds: Backgrounds,

    /// Stickers
    pub stickers: Vec<Sticker>,

    /// Traces 
    pub traces: Vec<TappingTrace>,
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
    Poster
}

impl Default for Mode {
    fn default() -> Self {
        Self::Poster
    }
}
