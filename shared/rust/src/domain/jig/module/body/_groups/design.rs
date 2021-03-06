use crate::domain::jig::module::body::{Background, Image, Instructions, ThemeChoice, Transform};
#[cfg(feature = "backend")]
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

/// The base content for design modules that don't need custom Sticker wrappers
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct BaseContent {
    /// The instructions for the module.
    pub instructions: Instructions,

    /// The module's theme.
    pub theme: ThemeChoice,

    /// Backgrounds
    pub backgrounds: Backgrounds,

    /// Stickers
    pub stickers: Vec<Sticker>,
}

#[derive(Clone, Default, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
/// Background
/// although it's simply a list of layers
/// the number of layers is predefined
/// and has special meaning from a UI perspective
pub struct Backgrounds {
    /// Layer 1
    pub layer_1: Option<Background>,
    /// Layer 2
    pub layer_2: Option<Background>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
/// Stickers are things that can be rendered and transformed
pub enum Sticker {
    /// Sprites
    Sprite(Sprite),
    /// Text
    Text(Text),
}

impl Sticker {
    /// Get the inner transform of a sticker
    pub fn transform(&self) -> &Transform {
        match self {
            Self::Sprite(sprite) => &sprite.transform,
            Self::Text(text) => &text.transform,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
/// Text are serialized text things
pub struct Text {
    /// the raw text
    pub value: String,
    /// The Transform
    pub transform: Transform,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
/// Sprites are a combo of image + transform
pub struct Sprite {
    /// The Image
    pub image: Image,
    /// The Transform
    pub transform: Transform,
    /// Effects
    pub effects: Vec<SpriteEffect>,

    /// Flip horizontal
    pub flip_horizontal: bool,

    /// Flip vertical
    pub flip_vertical: bool,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
/// Sprite Effects
pub enum SpriteEffect {
    /// Remove White
    RemoveWhite,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
/// Trace
pub struct Trace {
    /// The Transform
    pub transform: Transform,
    /// The Shape
    pub shape: TraceShape,
}

impl AsRef<Trace> for Trace {
    fn as_ref(&self) -> &Trace {
        self
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
/// Trace shape
pub enum TraceShape {
    /// width and height
    Rect(f64, f64),
    /// radius
    Ellipse(f64, f64),
    /// points
    Path(Vec<(f64, f64)>),
}
