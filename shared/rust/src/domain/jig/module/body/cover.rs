use crate::domain::jig::module::{
    body::{Backgrounds, Body, BodyExt, Instructions, Sticker, ThemeChoice},
    ModuleKind,
};
#[cfg(feature = "backend")]
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

/// The body for [`Cover`](crate::domain::jig::module::ModuleKind::Cover) modules.
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct ModuleData {
    /// The content
    pub content: Option<Content>,
}

impl BodyExt<()> for ModuleData {
    fn as_body(&self) -> Body {
        Body::Cover(self.clone())
    }

    fn is_complete(&self) -> bool {
        self.content.is_some()
    }

    fn kind() -> ModuleKind {
        ModuleKind::Cover
    }
    fn new_mode(_mode: ()) -> Self {
        ModuleData {
            content: Some(Content::default()),
        }
    }

    fn requires_choose_mode(&self) -> bool {
        false
    }
}

impl TryFrom<Body> for ModuleData {
    type Error = &'static str;

    fn try_from(body: Body) -> Result<Self, Self::Error> {
        match body {
            Body::Cover(data) => Ok(data),
            _ => Err("cannot convert body to cover!"),
        }
    }
}

/// The body for [`Cover`](crate::domain::jig::module::ModuleKind::Cover) modules.
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct Content {
    /// The instructions for the module.
    pub instructions: Instructions,

    /// The module's theme.
    pub theme: ThemeChoice,

    /// Backgrounds
    pub backgrounds: Backgrounds,

    /// Stickers
    pub stickers: Vec<Sticker>,
}
