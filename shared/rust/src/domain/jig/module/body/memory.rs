use crate::{
    domain::{
        audio::AudioId,
        image::ImageId,
        jig::module::{
            body::{Body, BodyExt, Instructions, ThemeChoice},
            ModuleKind,
        },
    },
    media::MediaLibrary,
};
#[cfg(feature = "backend")]
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

/// The body for [`Memory`](crate::domain::jig::module::ModuleKind::Memory) modules.
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct ModuleData {
    /// The content
    pub content: Option<Content>,
}

/// The content for [`Memory`](crate::domain::jig::module::ModuleKind::Memory) modules.
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct Content {
    /// The instructions for the module.
    pub instructions: Instructions,

    /// The mode the module uses.
    pub mode: Mode,

    /// The pairs of cards that make up the module.
    pub pairs: Vec<CardPair>,

    /// The ID of the module's theme.
    pub theme: ThemeChoice,
}

impl BodyExt for ModuleData {
    fn as_body(&self) -> Body {
        Body::MemoryGame(self.clone())
    }

    fn is_complete(&self) -> bool {
        self.content.is_some()
    }

    fn kind() -> ModuleKind {
        ModuleKind::Memory
    }
}

impl TryFrom<Body> for ModuleData {
    type Error = &'static str;

    fn try_from(body: Body) -> Result<Self, Self::Error> {
        match body {
            Body::MemoryGame(data) => Ok(data),
            _ => Err("cannot convert body to memory game!"),
        }
    }
}

/// A pair of cards
#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct CardPair(pub Card, pub Card);

/// An individual card.
#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub enum Card {
    // todo(@dakom): document this
    #[allow(missing_docs)]
    Text(String),

    // todo(@dakom): document this
    #[allow(missing_docs)]
    Image(Option<(ImageId, MediaLibrary)>),

    // todo(@dakom): document this
    #[allow(missing_docs)]
    Audio(Option<(AudioId, MediaLibrary)>),
}

/// What mode the module runs in.
#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
#[repr(i16)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
pub enum Mode {
    // todo(@dakom): document this
    #[allow(missing_docs)]
    Duplicate = 0,

    // todo(@dakom): document this
    #[allow(missing_docs)]
    WordsAndImages = 1,

    // todo(@dakom): document this
    #[allow(missing_docs)]
    BeginsWith = 2,

    // todo(@dakom): document this
    #[allow(missing_docs)]
    Lettering = 3,

    // todo(@dakom): document this
    #[allow(missing_docs)]
    Riddles = 4,

    // todo(@dakom): document this
    #[allow(missing_docs)]
    Opposites = 5,

    // todo(@dakom): document this
    #[allow(missing_docs)]
    Synonymns = 6,

    /// Translate from one language to another.
    Translate = 7,
}

impl Default for Mode {
    fn default() -> Self {
        Self::Duplicate
    }
}
