use crate::{
    domain::{audio::AudioId, image::ImageId},
    media::MediaLibrary,
};
#[cfg(feature = "backend")]
use paperclip::actix::Apiv2Schema;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{collections::HashSet, convert::TryFrom, fmt::Debug, hash::Hash};

/// Memory Game Body.
pub mod memory;

/// Poster Body.
pub mod poster;

/// Tapping Board Body.
pub mod tapping_board;

/// Cover Body.
pub mod cover;

/// Body kinds for Modules.
#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum Body {
    /// Module is a memory game, and has a memory game's body.
    MemoryGame(memory::ModuleData),

    /// Module is a poster, and has a poster's body.
    Poster(poster::ModuleData),

    /// Module is a tapping board, and has a tapping board's body.
    TappingBoard(tapping_board::ModuleData),

    /// Module is a [`Cover`](super::ModuleKind::Cover).
    ///
    /// DEPRECATED INFO: This exists as an empty enum because cover *needs* to exist, but it also isn't decided yet.
    Cover(cover::ModuleData),
}

impl Body {
    /// create a new Body for a given ModuleKind
    pub fn new(kind: super::ModuleKind) -> Self {
        match kind {
            super::ModuleKind::Cover => Self::Cover(cover::ModuleData::default()),
            super::ModuleKind::Memory => Self::MemoryGame(memory::ModuleData::default()),
            super::ModuleKind::Poster => Self::Poster(poster::ModuleData::default()),
            super::ModuleKind::TappingBoard => {
                Self::TappingBoard(tapping_board::ModuleData::default())
            }
            _ => unimplemented!("TODO!"),
        }
    }
}

/// Extension trait for interop
/// impl on inner body data
pub trait BodyExt<Mode: ModeExt, Step: StepExt>:
    TryFrom<Body> + Serialize + DeserializeOwned + Clone + Debug
{
    /// get choose mode list. By default it's the full list
    /// but that can be overridden to re-order or hide some modes
    fn choose_mode_list() -> Vec<Mode> {
        Mode::get_list()
    }

    /// get self as a Body
    fn as_body(&self) -> Body;

    /// is complete
    fn is_complete(&self) -> bool;

    /// get the kind from the type itself
    fn kind() -> super::ModuleKind;

    /// given a Mode, get a new Self
    /// will usually populate an inner .content
    fn new_mode(mode: Mode) -> Self;

    /// requires an additional step of choosing the mode
    fn requires_choose_mode(&self) -> bool;

    /// Get the current theme
    fn get_theme(&self) -> Option<ThemeChoice>;

    /// Set editor state step
    fn set_editor_state_step(&mut self, step: Step);
    /// Set editor state steps completed
    fn set_editor_state_steps_completed(&mut self, steps_completed: HashSet<Step>);
    /// Get editor state step
    fn get_editor_state_step(&self) -> Option<Step>;
    /// Get editor state steps completed
    fn get_editor_state_steps_completed(&self) -> Option<HashSet<Step>>;
    /// Insert a completed step
    fn insert_editor_state_step_completed(&mut self, step: Step) {
        if let Some(mut steps_completed) = self.get_editor_state_steps_completed() {
            steps_completed.insert(step);
            self.set_editor_state_steps_completed(steps_completed);
        }
    }
}

/// Extenstion trait for modes
pub trait ModeExt: Copy + Default + PartialEq + Eq + Hash {
    /// get a list of all the modes
    /// (becomes the default in Choose page, which can be overriden in BodyExt)
    fn get_list() -> Vec<Self>;

    /// get the mode itself as a string id
    fn as_str_id(&self) -> &'static str;
    /// for headers, labels, etc.
    fn label(&self) -> &'static str;
}

/// impl ModeExt for empty modes
/// this is a special case and should only be used
/// where the module genuinely ignores the mode
/// one example is the Cover module
impl ModeExt for () {
    fn get_list() -> Vec<Self> {
        vec![]
    }

    fn as_str_id(&self) -> &'static str {
        ""
    }

    fn label(&self) -> &'static str {
        ""
    }
}

impl Body {
    /// Gets this body's related [`ModuleKind`](super::ModuleKind)
    pub fn kind(&self) -> super::ModuleKind {
        match self {
            Self::Cover(_) => super::ModuleKind::Cover,
            Self::MemoryGame(_) => super::ModuleKind::Memory,
            Self::Poster(_) => super::ModuleKind::Poster,
            Self::TappingBoard(_) => super::ModuleKind::TappingBoard,
        }
    }
}

/* The following are things which are often used by multiple modules */

/// Generic editor state which must be preserved between sessions
/// Although these are saved to the db, they aren't relevant for playback
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct EditorState<STEP>
where
    STEP: StepExt,
{
    /// the current step
    pub step: STEP,

    /// the completed steps
    pub steps_completed: HashSet<STEP>,
}

/// This extension trait makes it possible to keep the Step
/// functionality generic and at a higher level than the module itself
pub trait StepExt: Copy + Default + PartialEq + Eq + Hash {
    /// Get the next step from current step
    fn next(&self) -> Option<Self>;
    /// Get the step as a number
    fn as_number(&self) -> usize;
    /// Label to display (will be localized)
    fn label(&self) -> &'static str;
    /// List of all available steps
    fn get_list() -> Vec<Self>;
    /// Get the step which is synonymous with "preview"
    /// TODO: this could probably be derived as a combo
    /// of get_list() and next() (i.e. the first step to return None)
    fn get_preview() -> Self;
    /// Auto-implemented, check whether current step is "preview"
    fn is_preview(&self) -> bool {
        *self == Self::get_preview()
    }
}

/// Theme Ids. Used in various modules
/// See the frontend extension trait for more info
#[derive(Clone, Copy, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
#[repr(i16)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
pub enum ThemeId {
    /// Empty theme (white bg, no text, etc.)
    Blank = 0,
    /// Blueish theme
    Chalkboard = 1,
    /// Orangeish theme
    HappyBrush = 2,
}

impl Default for ThemeId {
    fn default() -> Self {
        Self::Blank
    }
}

/// Theme choice, either jig or override
#[derive(Clone, Copy, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub enum ThemeChoice {
    /// Use the jig's theme
    Jig,

    /// Override it with a per-module choice
    Override(ThemeId),
}

impl Default for ThemeChoice {
    fn default() -> Self {
        Self::Jig
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
/// Audio
pub struct Audio {
    /// The Audio Id
    pub id: AudioId,
    /// The Media Library
    pub lib: MediaLibrary,
}

/// Instructions for a module.
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct Instructions {
    /// Text displayed in banner
    pub text: Option<String>,

    /// Audio played on module start
    pub audio: Option<Audio>,
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

#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
/// Background
pub enum Background {
    /// Color
    Color(rgb::RGBA8),
    /// Theme-based
    Theme(ThemeId),
    /// Any other image
    Image(Image),
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
/// Images need id and lib
pub struct Image {
    /// The Image Id
    pub id: ImageId,
    /// The MediaLibrary
    pub lib: MediaLibrary,
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
/// Vector of 3 floats
pub struct Vec3(pub [f64; 3]);

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
/// Vector of 4 floats, also used as a Quaternion
pub struct Vec4(pub [f64; 4]);

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
/// Visual Transform
pub struct Transform {
    /// Translation
    pub translation: Vec3,
    /// Rotation Quaternion
    pub rotation: Vec4,
    /// Scale for each axis
    pub scale: Vec3,
    /// Origin
    pub origin: Vec3,
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
