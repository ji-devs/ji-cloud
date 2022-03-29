use crate::domain::jig::module::body::{
    Audio, Background, Image, Instructions, ThemeId, Transform,
};
use serde::{Deserialize, Serialize};

/// Default text for `Text`
pub const DEFAULT_TEXT_VALUE: &str = "Shalom שָׁלוֹם";

/// The base content for design modules that don't need custom Sticker wrappers
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BaseContent {
    /// The instructions for the module.
    pub instructions: Instructions,

    /// The module's theme.
    pub theme: ThemeId,

    /// Backgrounds
    pub backgrounds: Backgrounds,

    /// Stickers
    pub stickers: Vec<Sticker>,
}

impl Default for BaseContent {
    fn default() -> Self {
        Self {
            instructions: Default::default(),
            theme: Default::default(),
            backgrounds: Default::default(),
            stickers: vec![Sticker::Text(Default::default())],
        }
    }
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
/// Stickers are things that can be rendered and transforme
pub enum Sticker {
    /// Sprites
    #[serde(alias = "sprite")]
    Sprite(Sprite),
    /// Text
    #[serde(alias = "text")]
    Text(Text),
    /// Video
    #[serde(alias = "video")]
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

impl Default for Text {
    fn default() -> Self {
        Self::from_str(DEFAULT_TEXT_VALUE)
    }
}

impl Text {
    // value is the ready slate json and str is just a string of text

    /// Create a new Text from a str
    pub fn from_str(text: &str) -> Self {
        Self::from_value(Self::value_from_str(text))
    }

    /// Create a new Text from value
    pub fn from_value(value: String) -> Self {
        Self {
            value,
            transform: Transform::identity(),
        }
    }

    /// Get a value string from a str
    pub fn value_from_str(text: &str) -> String {
        format!(
            r#"{{"version":"0.1.0","content":[{{"children":[{{"text":"{}","element":"H1"}}]}}]}}"#,
            text
        )
    }
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
#[serde(rename_all = "snake_case")]
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
    #[serde(alias = "youtube")]
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
    #[serde(alias = "wrong")]
    Wrong,
    /// Correct (green color)
    #[serde(alias = "correct")]
    Correct,
    /// Regular (blue color)
    #[serde(alias = "regular")]
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
    #[serde(alias = "rect")]
    Rect(f64, f64),
    /// radius
    #[serde(alias = "ellipse")]
    Ellipse(f64, f64),
    /// points
    #[serde(alias = "path")]
    Path(Vec<(f64, f64)>),
    /// explicit path commands
    /// corresponds to SVG spec: https://svgwg.org/svg2-draft/paths.html#TheDProperty
    /// the second parameter indicates whether it's absolute (true) or relative (false)
    #[serde(alias = "pathCommands")]
    PathCommands(Vec<(PathCommand, bool)>),
}

#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum PathCommand {
    /// https://svgwg.org/svg2-draft/paths.html#PathDataMovetoCommands
    MoveTo(f64, f64),
    /// https://svgwg.org/svg2-draft/paths.html#PathDataLinetoCommands
    ClosePath,
    /// https://svgwg.org/svg2-draft/paths.html#PathDataLinetoCommands
    LineTo(f64, f64),
    /// https://svgwg.org/svg2-draft/paths.html#PathDataLinetoCommands
    HorizontalLineTo(f64),
    /// https://svgwg.org/svg2-draft/paths.html#PathDataLinetoCommands
    VerticalLineTo(f64),
    /// https://svgwg.org/svg2-draft/paths.html#PathDataCubicBezierCommands
    CurveTo(f64, f64, f64, f64, f64, f64),
    /// https://svgwg.org/svg2-draft/paths.html#PathDataCubicBezierCommands
    SmoothCurveTo(f64, f64, f64, f64),
    /// https://svgwg.org/svg2-draft/paths.html#PathDataQuadraticBezierCommands
    QuadCurveTo(f64, f64, f64, f64),
    /// https://svgwg.org/svg2-draft/paths.html#PathDataQuadraticBezierCommands
    SmoothQuadCurveTo(f64, f64),
    /// https://svgwg.org/svg2-draft/paths.html#PathDataEllipticalArcCommands
    ArcTo(f64, f64, f64, f64, f64, f64, f64),
}
