use crate::domain::jig::module::{
    body::{EditorState, StepExt, Backgrounds, Body, BodyExt, Instructions, Sticker, ThemeChoice},
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

impl BodyExt<Mode> for ModuleData {
    fn as_body(&self) -> Body {
        Body::Poster(self.clone())
    }

    fn is_complete(&self) -> bool {
        self.content.is_some()
    }

    fn kind() -> ModuleKind {
        ModuleKind::Poster
    }
    fn new_mode(mode: Mode) -> Self {
        ModuleData {
            content: Some(Content {
                mode,
                ..Content::default()
            }),
        }
    }

    fn requires_choose_mode(&self) -> bool {
        self.content.is_none()
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
    /// The editor state
    pub editor_state: EditorState<Step>,

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

/// The Steps
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Step {
    /// Step 1
    One,
    /// Step 2
    Two,
    /// Step 3
    Three,
    /// Step 4
    Four,
}

impl Default for Step {
    fn default() -> Self {
        Self::One
    }
}

impl StepExt for Step {
    fn next(&self) -> Option<Self> {
        match self {
            Self::One => Some(Self::Two),
            Self::Two => Some(Self::Three),
            Self::Three => Some(Self::Four),
            Self::Four => None,
        }
    }

    fn as_number(&self) -> usize {
        match self {
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
        }
    }

    fn label(&self) -> &'static str {
        //TODO - localizaton
        const STR_THEMES:&'static str = "Themes";
        const STR_BACKGROUND:&'static str = "Background";
        const STR_CONTENT:&'static str = "Content";
        const STR_PREVIEW:&'static str = "Preview";

        match self {
            Self::One => STR_THEMES,
            Self::Two => STR_BACKGROUND,
            Self::Three => STR_CONTENT,
            Self::Four => STR_PREVIEW,
        }
    }

    fn get_list() -> Vec<Self> {
        vec![
            Self::One,
            Self::Two,
            Self::Three,
            Self::Four,
        ]
    }
    fn get_preview() -> Self {
        Self::Four
    }
}
