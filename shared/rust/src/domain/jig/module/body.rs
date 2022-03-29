use super::ModuleKind;
use crate::{
    domain::{audio::AudioId, image::ImageId},
    media::MediaLibrary,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{collections::HashSet, convert::TryFrom, fmt::Debug, hash::Hash};

/// Memory Game Body.
pub mod memory;

/// Talking Poster Body.
pub mod poster;

/// Video Body.
pub mod video;

/// Listen & Learn Body.
pub mod tapping_board;

/// Drag & Drop Body.
pub mod drag_drop;

/// Cover Body.
pub mod cover;

/// Resource Cover Body.
pub mod resource_cover;

/// Flashcards .
pub mod flashcards;

/// Quiz Game
pub mod card_quiz;

/// Matching
pub mod matching;

/// Legacy
pub mod legacy;

/// Groups that share types
pub mod _groups;

/// Body kinds for Modules.
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum Body {
    /// Module is a memory game, and has a memory game's body.
    MemoryGame(memory::ModuleData),

    /// Module is matching game, and has a matching game's body.
    Matching(matching::ModuleData),

    /// Module is flashcards, and has a flashcard's body.
    Flashcards(flashcards::ModuleData),

    /// Module is a quiz game, and has a quiz game's body.
    CardQuiz(card_quiz::ModuleData),

    /// Module is a poster, and has a talking poster's body.
    Poster(poster::ModuleData),

    /// Module is a Video, and has a video body.
    Video(video::ModuleData),

    /// Module is a Listen & Learn, and has a Listen & Learn's body.
    TappingBoard(tapping_board::ModuleData),

    /// Module is a drag & drop, and has a drag & drop's body.
    DragDrop(drag_drop::ModuleData),

    /// Module is a [`Cover`](super::ModuleKind::Cover).
    ///
    /// Cover for Module type
    Cover(cover::ModuleData),

    /// Module is a Resource Cover.
    ResourceCover(resource_cover::ModuleData),

    /// Module is a legacy, and has a legacy's body.
    Legacy(legacy::ModuleData),
}

impl Body {
    /// create a new Body for a given ModuleKind
    pub fn new(kind: super::ModuleKind) -> Self {
        match kind {
            super::ModuleKind::Cover => Self::Cover(Default::default()),
            super::ModuleKind::ResourceCover => Self::ResourceCover(Default::default()),
            super::ModuleKind::Memory => Self::MemoryGame(Default::default()),
            super::ModuleKind::CardQuiz => Self::CardQuiz(Default::default()),
            super::ModuleKind::Flashcards => Self::Flashcards(Default::default()),
            super::ModuleKind::Matching => Self::Matching(Default::default()),
            super::ModuleKind::Poster => Self::Poster(Default::default()),
            super::ModuleKind::Video => Self::Video(Default::default()),
            super::ModuleKind::TappingBoard => Self::TappingBoard(Default::default()),
            super::ModuleKind::DragDrop => Self::DragDrop(Default::default()),
            super::ModuleKind::Legacy => Self::Legacy(Default::default()),
            _ => unimplemented!("TODO!"),
        }
    }

    /// Convert this container to a Body wrapper of a specific kind
    pub fn convert_to_body(&self, kind: ModuleKind) -> Result<Self, &'static str> {
        match self {
            Self::MemoryGame(data) => data.convert_to_body(kind),
            Self::Matching(data) => data.convert_to_body(kind),
            Self::Flashcards(data) => data.convert_to_body(kind),
            Self::CardQuiz(data) => data.convert_to_body(kind),
            Self::Poster(data) => data.convert_to_body(kind),
            Self::Video(data) => data.convert_to_body(kind),
            Self::TappingBoard(data) => data.convert_to_body(kind),
            Self::DragDrop(data) => data.convert_to_body(kind),
            Self::Cover(data) => data.convert_to_body(kind),
            Self::ResourceCover(data) => data.convert_to_body(kind),
            Self::Legacy(data) => data.convert_to_body(kind),
        }
    }
}

/// Extension trait for interop
/// impl on inner body data
pub trait BodyExt<Mode: ModeExt, Step: StepExt>:
    BodyConvert + TryFrom<Body> + Serialize + DeserializeOwned + Clone + Debug
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

    /// is legacy
    fn is_legacy() -> bool {
        false
    }

    /// should wait for manual phase change to Init
    fn has_preload() -> bool {
        false
    }

    /// get the kind from the type itself
    fn kind() -> super::ModuleKind;

    /// given a Mode, get a new Self
    /// will usually populate an inner .content
    fn new_with_mode_and_theme(mode: Mode, theme_id: ThemeId) -> Self;

    /// Fetch the mode for this module
    fn mode(&self) -> Option<Mode>;

    /// requires an additional step of choosing the mode
    fn requires_choose_mode(&self) -> bool;

    /// Get the current theme
    fn get_theme(&self) -> Option<ThemeId>;

    /// Set the current theme
    fn set_theme(&mut self, theme_id: ThemeId);

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

    /// Convert this inner data to a Body wrapper of a specific kind
    fn convert_to_body(&self, kind: ModuleKind) -> Result<Body, &'static str> {
        match kind {
            ModuleKind::Memory => Ok(Body::MemoryGame(self.convert_to_memory()?)),
            ModuleKind::Matching => Ok(Body::Matching(self.convert_to_matching()?)),
            ModuleKind::Flashcards => Ok(Body::Flashcards(self.convert_to_flashcards()?)),
            ModuleKind::CardQuiz => Ok(Body::CardQuiz(self.convert_to_card_quiz()?)),
            ModuleKind::Poster => Ok(Body::Poster(self.convert_to_poster()?)),
            ModuleKind::Video => Ok(Body::Video(self.convert_to_video()?)),
            ModuleKind::TappingBoard => Ok(Body::TappingBoard(self.convert_to_tapping_board()?)),
            ModuleKind::DragDrop => Ok(Body::DragDrop(self.convert_to_drag_drop()?)),
            ModuleKind::Cover => Ok(Body::Cover(self.convert_to_cover()?)),
            ModuleKind::ResourceCover => Ok(Body::ResourceCover(self.convert_to_resource_cover()?)),
            ModuleKind::Legacy => Ok(Body::Legacy(self.convert_to_legacy()?)),
            _ => unimplemented!(
                "cannot convert from {} to {}",
                Self::kind().as_str(),
                kind.as_str()
            ),
        }
    }
}

/// These will all error by default.
/// Modules that can be converted between eachother must override
/// The relevant methods
pub trait BodyConvert {
    /// Get a list of valid conversion targets
    fn convertable_list() -> Vec<ModuleKind> {
        Vec::new()
    }
    /// Memory game
    fn convert_to_memory(&self) -> Result<memory::ModuleData, &'static str> {
        Err("cannot convert to memory game!")
    }
    /// Matching
    fn convert_to_matching(&self) -> Result<matching::ModuleData, &'static str> {
        Err("cannot convert to matching!")
    }
    /// Flashcards
    fn convert_to_flashcards(&self) -> Result<flashcards::ModuleData, &'static str> {
        Err("cannot convert to matching!")
    }
    /// Quiz game
    fn convert_to_card_quiz(&self) -> Result<card_quiz::ModuleData, &'static str> {
        Err("cannot convert to quiz game!")
    }
    /// Talking Poster
    fn convert_to_poster(&self) -> Result<poster::ModuleData, &'static str> {
        Err("cannot convert to talking poster!")
    }
    /// Listen & Learn
    fn convert_to_tapping_board(&self) -> Result<tapping_board::ModuleData, &'static str> {
        Err("cannot convert to Listen & Learn!")
    }
    /// Drag & Drop
    fn convert_to_drag_drop(&self) -> Result<drag_drop::ModuleData, &'static str> {
        Err("cannot convert to drag & drop!")
    }
    /// Cover
    fn convert_to_cover(&self) -> Result<cover::ModuleData, &'static str> {
        Err("cannot convert to cover!")
    }
    /// Resource Cover
    fn convert_to_resource_cover(&self) -> Result<resource_cover::ModuleData, &'static str> {
        Err("cannot convert to resource cover!")
    }
    /// Video
    fn convert_to_video(&self) -> Result<video::ModuleData, &'static str> {
        Err("cannot convert to video!")
    }
    /// Legacy
    fn convert_to_legacy(&self) -> Result<legacy::ModuleData, &'static str> {
        Err("cannot convert to legacy!")
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

    /// Image tag filters for search
    /// The actual ImageTag enum variants are in components
    fn image_tag_filters(&self) -> Option<Vec<i16>> {
        None
    }

    /// Image tag priorities for search
    /// The actual ImageTag enum variants are in components
    fn image_tag_priorities(&self) -> Option<Vec<i16>> {
        None
    }
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
            Self::ResourceCover(_) => super::ModuleKind::ResourceCover,
            Self::MemoryGame(_) => super::ModuleKind::Memory,
            Self::Flashcards(_) => super::ModuleKind::Flashcards,
            Self::CardQuiz(_) => super::ModuleKind::CardQuiz,
            Self::Matching(_) => super::ModuleKind::Matching,
            Self::Poster(_) => super::ModuleKind::Poster,
            Self::Video(_) => super::ModuleKind::Video,
            Self::TappingBoard(_) => super::ModuleKind::TappingBoard,
            Self::DragDrop(_) => super::ModuleKind::DragDrop,
            Self::Legacy(_) => super::ModuleKind::Legacy,
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
pub trait StepExt: Copy + Default + PartialEq + Eq + Hash + Debug {
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

/// impl StepExt for empty steps
/// this is a special case and should only be used
/// where the module genuinely ignores the step
/// one example is the Legacy module
impl StepExt for () {
    fn next(&self) -> Option<Self> {
        None
    }
    fn as_number(&self) -> usize {
        0
    }
    fn label(&self) -> &'static str {
        ""
    }
    fn get_list() -> Vec<Self> {
        Vec::new()
    }
    fn get_preview() -> Self {
        ()
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
/// Audio
pub struct Audio {
    /// The Audio Id
    pub id: AudioId,
    /// The Media Library
    pub lib: MediaLibrary,
}

/// Instructions for a module.
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct Instructions {
    /// Text displayed in banner
    pub text: Option<String>,

    /// Audio played on module start
    pub audio: Option<Audio>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
/// Background
pub enum Background {
    /// Color
    Color(Option<rgb::RGBA8>),
    /// Any other image
    Image(Image),
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
/// Images need id and lib
pub struct Image {
    /// The Image Id
    pub id: ImageId,
    /// The MediaLibrary
    pub lib: MediaLibrary,
}

#[derive(Clone, Default, Serialize, Deserialize, Debug, PartialEq)]
/// Vector of 2 floats
pub struct Vec2(pub [f64; 2]);

impl From<(f64, f64)> for Vec2 {
    fn from((x, y): (f64, f64)) -> Self {
        Self([x, y])
    }
}

impl From<Vec2> for (f64, f64) {
    fn from(v: Vec2) -> Self {
        (v.0[0], v.0[1])
    }
}

#[derive(Clone, Default, Serialize, Deserialize, Debug, PartialEq)]
/// Vector of 3 floats
pub struct Vec3(pub [f64; 3]);

impl From<(f64, f64, f64)> for Vec3 {
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        Self([x, y, z])
    }
}

impl From<Vec3> for (f64, f64, f64) {
    fn from(v: Vec3) -> Self {
        (v.0[0], v.0[1], v.0[2])
    }
}

#[derive(Clone, Default, Serialize, Deserialize, Debug, PartialEq)]
/// Vector of 4 floats, also used as a Quaternion
pub struct Vec4(pub [f64; 4]);

impl From<(f64, f64, f64, f64)> for Vec4 {
    fn from((x, y, z, w): (f64, f64, f64, f64)) -> Self {
        Self([x, y, z, w])
    }
}

impl From<Vec4> for (f64, f64, f64, f64) {
    fn from(v: Vec4) -> Self {
        (v.0[0], v.0[1], v.0[2], v.0[3])
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
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

impl Transform {
    /// Create a new Transform
    pub fn identity() -> Self {
        Self {
            translation: Vec3([0.0, 0.0, 0.0]),
            rotation: Vec4([0.0, 0.0, 0.0, 1.0]),
            scale: Vec3([1.0, 1.0, 1.0]),
            origin: Vec3([0.0, 0.0, 0.0]),
        }
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::identity()
    }
}

/// Theme Ids. Used in various modules
/// See the frontend extension trait for more info
#[derive(Clone, Copy, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[repr(i16)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
pub enum ThemeId {
    #[allow(missing_docs)]
    Blank,
    #[allow(missing_docs)]
    Jigzi,
    #[allow(missing_docs)]
    Chalkboard,
    #[allow(missing_docs)]
    MyNotebook,
    #[allow(missing_docs)]
    BackToSchool,
    #[allow(missing_docs)]
    MyWorkspace,
    #[allow(missing_docs)]
    Comix,
    #[allow(missing_docs)]
    Surreal,
    #[allow(missing_docs)]
    Abstract,
    #[allow(missing_docs)]
    Denim,
    #[allow(missing_docs)]
    HappyBrush,
    #[allow(missing_docs)]
    Graffiti,
    #[allow(missing_docs)]
    JewishText,
    #[allow(missing_docs)]
    ShabbatShalom,
    #[allow(missing_docs)]
    RoshHashana,
    #[allow(missing_docs)]
    AppleWithHoney,
    #[allow(missing_docs)]
    Pomegranate,
    #[allow(missing_docs)]
    YomKippur,
    #[allow(missing_docs)]
    HappySukkot,
    #[allow(missing_docs)]
    Sukkot,
    #[allow(missing_docs)]
    IlluminatingHanukkah,
    #[allow(missing_docs)]
    Chanukah,
    #[allow(missing_docs)]
    ChanukahLights,
    #[allow(missing_docs)]
    Purim,
    #[allow(missing_docs)]
    PurimFeast,
    #[allow(missing_docs)]
    PurimSweets,
    #[allow(missing_docs)]
    HappyPassover,
    #[allow(missing_docs)]
    PassoveMatza,
    #[allow(missing_docs)]
    PassoverSeder,
    #[allow(missing_docs)]
    HappyShavuot,
    #[allow(missing_docs)]
    ShavuotDishes,
    #[allow(missing_docs)]
    ShavuotFields,
    #[allow(missing_docs)]
    OurIsrael,
    #[allow(missing_docs)]
    Israel,
    #[allow(missing_docs)]
    JerusalemCity,
    #[allow(missing_docs)]
    JerusalemWall,
    #[allow(missing_docs)]
    LovelySpring,
    #[allow(missing_docs)]
    Spring,
    #[allow(missing_docs)]
    WatermelonSummer,
    #[allow(missing_docs)]
    SummerPool,
    #[allow(missing_docs)]
    ExcitingFall,
    #[allow(missing_docs)]
    Autumn,
    #[allow(missing_docs)]
    WinterSnow,
    #[allow(missing_docs)]
    IceAge,
    #[allow(missing_docs)]
    LostInSpace,
    #[allow(missing_docs)]
    Space,
    #[allow(missing_docs)]
    Camping,
    #[allow(missing_docs)]
    HappyBirthday,
    #[allow(missing_docs)]
    Jungle,
    #[allow(missing_docs)]
    OurPlanet,
    #[allow(missing_docs)]
    Theater,
    #[allow(missing_docs)]
    Travel,
}

impl Default for ThemeId {
    fn default() -> Self {
        Self::Blank
    }
}
