use crate::domain::jig::module::{
    body::{Backgrounds, Body, BodyExt, Instructions, Sticker, ThemeId},
    ModuleKind,
};
#[cfg(feature = "backend")]
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

/// The body for [`Poster`](crate::domain::jig::module::ModuleKind::Poster) modules.
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct ModuleData {
    /// The content
    pub content: Option<Content>,
}

impl BodyExt for ModuleData {
    fn as_body(&self) -> Body {
        Body::Poster(self.clone())
    }

    fn is_complete(&self) -> bool {
        self.content.is_some()
    }

    fn kind() -> ModuleKind {
        ModuleKind::Poster
    }
}

impl TryFrom<Body> for ModuleData {
    type Error = &'static str;

    fn try_from(body: Body) -> Result<Self, Self::Error> {
        match body {
            Body::Poster(data) => Ok(data),
            _ => Err("cannot convert body to poster!"),
        }
    }
}

/// The body for [`Poster`](crate::domain::jig::module::ModuleKind::Poster) modules.
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
