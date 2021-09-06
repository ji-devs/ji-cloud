use crate::domain::jig::module::body::{
    Audio, Background, Image, Instructions, ThemeChoice, Transform,
};
use serde::{Deserialize, Serialize};

/// The base content for design modules that don't need custom Sticker wrappers
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
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
/// Stickers are things that can be rendered and transformed
pub enum Sticker {
    /// Sprites
    Sprite(Sprite),
    /// Text
    Text(Text),
    /// Video
    Video(Video),
}

impl Sticker {
    /// Get the inner transform of a sticker
    pub fn transform(&self) -> &Transform {
        match self {
            Self::Sprite(sprite) => &sprite.transform,
            Self::Text(text) => &text.transform,
            Self::Video(video) => &video.transform,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
/// Text are serialized text things
pub struct Text {
    /// the raw text
    pub value: String,
    /// The Transform
    pub transform: Transform,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
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
/// Sprite Effects
pub enum SpriteEffect {
    /// Remove White
    RemoveWhite,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
/// Video
/// Text are serialized text things
pub struct Video {
    /// The video host
    pub host: VideoHost,

    /// Transforms
    pub transform: Transform,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
/// Host of video
pub enum VideoHost {
    /// YouTube
    Youtube(YoutubeUrl),
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
/// YouTube host video url
pub struct YoutubeUrl(pub String);

/// Youtube url parse error
pub type YoutubeUrlError = String;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
/// Trace
pub struct Trace {
    /// The Transform
    pub transform: Transform,
    /// The Shape
    pub shape: TraceShape,
    /// The Kind
    pub kind: TraceKind,
    /// Optional audio associated with this trace
    pub audio: Option<Audio>,
    /// Optional text associated with this trace
    pub text: Option<String>,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq)]
/// Trace kind
pub enum TraceKind {
    /// Wrong (red color)
    Wrong,
    /// Correct (green color)
    Correct,
    /// Regular (blue color)
    Regular,
}

impl AsRef<Trace> for Trace {
    fn as_ref(&self) -> &Trace {
        self
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
/// Trace shape
pub enum TraceShape {
    /// width and height
    Rect(f64, f64),
    /// radius
    Ellipse(f64, f64),
    /// points
    Path(Vec<(f64, f64)>),
}
