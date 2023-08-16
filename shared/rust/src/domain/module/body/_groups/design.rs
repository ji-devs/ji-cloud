use crate::domain::module::body::{Audio, Background, Image, ModuleAssist, ThemeId, Transform};
use derive_setters::Setters;
use mymacros::{Deserialize, Serialize};

/// Default text for `Text`
pub const DEFAULT_TEXT_VALUE: &str = "Shalom שָׁלוֹם";

/// The base content for design modules that don't need custom Sticker wrappers
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BaseContent {
    /// The instructions for the module.
    pub instructions: ModuleAssist,

    /// The feedback for the module.
    #[serde(default)]
    pub feedback: ModuleAssist,

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
            feedback: Default::default(),
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
    // #[serde(alias = "sprite")]
    Sprite(Sprite),
    /// Text
    // #[serde(alias = "text")]
    Text(Text),
    /// Embed
    // #[serde(alias = "embed")]
    Embed(Embed),
}

impl Sticker {
    /// Get the inner transform of a sticker
    pub fn transform(&self) -> &Transform {
        match self {
            Self::Sprite(sprite) => &sprite.transform,
            Self::Text(text) => &text.transform,
            Self::Embed(embed) => &embed.transform,
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
// #[serde(rename_all = "snake_case")]
/// Sprite Effects
pub enum SpriteEffect {
    /// Remove White
    RemoveWhite,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
/// Embed
/// Text are serialized text things
pub struct Embed {
    /// The embed host
    pub host: EmbedHost,

    /// Transforms
    pub transform: Transform,
}

/// what to do when done
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum DoneAction {
    /// loop
    Loop,

    /// move on to next activity
    Next,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
/// Host of embed
pub enum EmbedHost {
    /// YouTube
    // #[serde(alias = "youtube")]
    Youtube(YoutubeEmbed),

    /// Vimeo
    Vimeo(VimeoEmbed),

    /// Google doc
    GoogleDoc(GoogleDocsEmbed),

    /// Google form
    GoogleForm(GoogleFormsEmbed),

    /// Google sheets
    GoogleSheet(GoogleSheetsEmbed),

    /// Google slide
    GoogleSlide(GoogleSlidesEmbed),

    /// Edpuzzle
    Edpuzzle(EdpuzzleEmbed),

    /// Puzzel
    Puzzel(PuzzelEmbed),

    /// Quizlet
    Quizlet(QuizletEmbed),

    /// Thinglink
    Thinglink(ThinglinkEmbed),

    /// Sutori
    Sutori(SutoriEmbed),
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Setters)]
/// YouTube host embed
pub struct YoutubeEmbed {
    /// url of the YouTube embed
    pub url: YoutubeUrl,

    /// start at second
    pub start_at: Option<u32>,

    /// end at second
    pub end_at: Option<u32>,

    /// show captions
    pub captions: bool,

    /// play with sound
    pub muted: bool,

    /// autoplay
    pub autoplay: bool,

    /// what to do when done
    pub done_action: Option<DoneAction>,
}

impl YoutubeEmbed {
    /// creates a new YoutubeEmbed
    pub fn new(url: YoutubeUrl) -> Self {
        Self {
            url,
            start_at: None,
            end_at: None,
            captions: false,
            muted: false,
            autoplay: false,
            done_action: None,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
/// YouTube host embed url
pub struct YoutubeUrl(pub String);

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Setters)]
/// Vimeo host embed
pub struct VimeoEmbed {
    /// url of the Vimeo embed
    pub url: VimeoUrl,

    /// start at second
    pub start_at: Option<u32>,

    /// end at second
    pub end_at: Option<u32>,

    /// show captions
    pub captions: bool,

    /// play with sound
    pub muted: bool,

    /// autoplay
    pub autoplay: bool,

    /// what to do when done
    pub done_action: Option<DoneAction>,
}

impl VimeoEmbed {
    /// creates a new VimeoEmbed
    pub fn new(url: VimeoUrl) -> Self {
        Self {
            url,
            start_at: None,
            end_at: None,
            captions: false,
            muted: false,
            autoplay: false,
            done_action: None,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
/// Vimeo host embed url
pub struct VimeoUrl(pub String);

impl EmbedHost {
    /// Convert youtube host url to a string
    pub fn get_url_string(&self) -> String {
        match &self {
            EmbedHost::Youtube(youtube_video) => {
                let YoutubeUrl(url) = &youtube_video.url;
                url.to_string()
            }
            EmbedHost::Vimeo(vimeo_video) => {
                let VimeoUrl(url) = &vimeo_video.url;
                url.to_string()
            }
            EmbedHost::GoogleDoc(google_doc) => {
                let GoogleDocId(url) = &google_doc.url;
                url.to_string()
            }
            EmbedHost::GoogleForm(google_form) => {
                let GoogleFormId(url) = &google_form.url;
                url.to_string()
            }
            EmbedHost::GoogleSheet(google_sheet) => {
                let GoogleSheetId(url) = &google_sheet.url;
                url.to_string()
            }
            EmbedHost::GoogleSlide(google_slide) => {
                let GoogleSlideId(url) = &google_slide.url;
                url.to_string()
            }
            EmbedHost::Edpuzzle(_) => todo!(),
            EmbedHost::Puzzel(_) => todo!(),
            EmbedHost::Quizlet(quizlet) => {
                let QuizletId(url) = &quizlet.url;
                url.to_string()
            }
            EmbedHost::Thinglink(thinglink) => {
                let ThinglinkId(url) = &thinglink.url;
                url.to_string()
            }
            EmbedHost::Sutori(sutori) => {
                let SutoriId(url) = &sutori.url;
                url.to_string()
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
/// Google docs host embed url
pub struct GoogleDocsEmbed {
    /// url of the YouTube video
    pub url: GoogleDocId,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
/// GoogleDoc host google doc url
pub struct GoogleDocId(pub String);

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
/// Google forms host embed url
pub struct GoogleFormsEmbed {
    /// url of the YouTube video
    pub url: GoogleFormId,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
/// GoogleForm host google form url
pub struct GoogleFormId(pub String);

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
/// Google sheets host embed url
pub struct GoogleSheetsEmbed {
    /// url of the YouTube video
    pub url: GoogleSheetId,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
/// GoogleSheet host google sheet url
pub struct GoogleSheetId(pub String);

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
/// Google slides host embed url
pub struct GoogleSlidesEmbed {
    /// url of the YouTube video
    pub url: GoogleSlideId,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
/// GoogleSlide host google slide url
pub struct GoogleSlideId(pub String);

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
/// Edpuzzle host embed url
pub struct EdpuzzleEmbed {
    /// url of the Edpuzzle video
    pub url: EdpuzzleId,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
/// Edpuzzle host google sheet url
pub struct EdpuzzleId(pub String);

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
/// Puzzel host embed url
pub struct PuzzelEmbed {
    /// url of the Puzzel video
    pub url: PuzzelId,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
/// Puzzel host google sheet url
pub struct PuzzelId(pub String);

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
/// Quizlet host embed url
pub struct QuizletEmbed {
    /// url of the Quizlet video
    pub url: QuizletId,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
/// Quizlet host google sheet url
pub struct QuizletId(pub String);

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
/// Thinglink host embed url
pub struct ThinglinkEmbed {
    /// url of the Thinglink video
    pub url: ThinglinkId,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
/// Thinglink host google sheet url
pub struct ThinglinkId(pub String);

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
/// Sutori host embed url
pub struct SutoriEmbed {
    /// url of the Sutori video
    pub url: SutoriId,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
/// Sutori host google sheet url
pub struct SutoriId(pub String);

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
    // #[serde(alias = "wrong")]
    Wrong,
    /// Correct (green color)
    // #[serde(alias = "correct")]
    Correct,
    /// Regular (blue color)
    // #[serde(alias = "regular")]
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
    // #[serde(alias = "rect")]
    Rect(f64, f64),
    /// radius
    // #[serde(alias = "ellipse")]
    Ellipse(f64, f64),
    /// points
    // #[serde(alias = "path")]
    Path(Vec<(f64, f64)>),
    /// explicit path commands
    /// corresponds to SVG spec: https://svgwg.org/svg2-draft/paths.html#TheDProperty
    /// the second parameter indicates whether it's absolute (true) or relative (false)
    // #[serde(alias = "pathCommands")]
    PathCommands(Vec<(PathCommand, bool)>),
}

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq)]
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

impl miniserde::Deserialize for PathCommand {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

impl miniserde::Serialize for PathCommand {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        todo!()
    }
}
