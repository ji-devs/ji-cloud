use crate::domain::jig::module::{
    body::{
        Audio, Body, BodyConvert, BodyExt, Instructions, ModeExt, StepExt, ThemeId, Transform,
        _groups::design::{Backgrounds, Sticker, Trace},
    },
    ModuleKind,
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::convert::TryFrom;

mod play_settings;
pub use play_settings::*;

use super::_groups::design::Text;

/// The body for [`DragDrop`](crate::domain::jig::module::ModuleKind::DragDrop) modules.
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct ModuleData {
    /// The content
    pub content: Option<Content>,
}

impl BodyExt<Mode, Step> for ModuleData {
    fn as_body(&self) -> Body {
        Body::DragDrop(self.clone())
    }

    fn is_complete(&self) -> bool {
        self.content.is_some()
    }

    fn kind() -> ModuleKind {
        ModuleKind::DragDrop
    }

    fn new_with_mode_and_theme(mode: Mode, theme: ThemeId) -> Self {
        ModuleData {
            content: Some(Content {
                mode,
                theme,
                items: vec![Item {
                    sticker: Sticker::Text(Text::default()),
                    kind: ItemKind::Static,
                }],
                ..Default::default()
            }),
        }
    }

    fn mode(&self) -> Option<Mode> {
        self.content.as_ref().map(|c| c.mode.clone())
    }

    fn requires_choose_mode(&self) -> bool {
        self.content.is_none()
    }

    fn set_editor_state_step(&mut self, step: Step) {
        if let Some(content) = self.content.as_mut() {
            content.editor_state.step = step;
        }
    }
    fn set_editor_state_steps_completed(&mut self, steps_completed: HashSet<Step>) {
        if let Some(content) = self.content.as_mut() {
            content.editor_state.steps_completed = steps_completed;
        }
    }

    fn get_editor_state_step(&self) -> Option<Step> {
        self.content
            .as_ref()
            .map(|content| content.editor_state.step)
    }

    fn get_editor_state_steps_completed(&self) -> Option<HashSet<Step>> {
        self.content
            .as_ref()
            .map(|content| content.editor_state.steps_completed.clone())
    }

    fn set_theme(&mut self, theme_id: ThemeId) {
        if let Some(content) = self.content.as_mut() {
            content.theme = theme_id;
        }
    }

    fn get_theme(&self) -> Option<ThemeId> {
        self.content.as_ref().map(|content| content.theme)
    }
}

impl BodyConvert for ModuleData {}

impl TryFrom<Body> for ModuleData {
    type Error = &'static str;

    fn try_from(body: Body) -> Result<Self, Self::Error> {
        match body {
            Body::DragDrop(data) => Ok(data),
            _ => Err("cannot convert body to drag & drop!"),
        }
    }
}

/// The body for [`DragDrop`](crate::domain::jig::module::ModuleKind::DragDrop) modules.
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct Content {
    /// The instructions for the module.
    pub instructions: Instructions,

    /// The module's theme.
    pub theme: ThemeId,

    /// Backgrounds
    pub backgrounds: Backgrounds,

    /// Items (wrapper around Sticker and metadata)
    pub items: Vec<Item>,

    /// List of targets for items
    ///
    /// Each item can possibly have multiple targets
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub item_targets: Vec<Item>,

    /// The editor state
    pub editor_state: EditorState,

    /// The mode
    pub mode: Mode,

    /// target areas
    pub target_areas: Vec<TargetArea>,

    /// play settings
    pub play_settings: PlaySettings,

    /// The feedback for the module.
    pub feedback: Instructions,
}

/// Editor state
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct EditorState {
    /// the current step
    pub step: Step,

    /// the completed steps
    pub steps_completed: HashSet<Step>,
}

/// drag & drop sticker w/ metadata
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Item {
    /// the sticker
    pub sticker: Sticker,

    /// the kind
    pub kind: ItemKind,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
/// The mode
pub enum ItemKind {
    /// Just part of the scene
    Static,
    /// One of the draggables
    Interactive(Interactive),
}

/// drag & drop sticker w/ metadata
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Interactive {
    /// audio
    pub audio: Option<Audio>,

    /// target transform
    pub target_transform: Option<Transform>,
}

/// drag & drop trace w/ metadata
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TargetArea {
    /// the trace
    pub trace: Trace,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
/// The mode
pub enum Mode {
    #[allow(missing_docs)]
    SettingTable,
    #[allow(missing_docs)]
    Sorting,
    #[allow(missing_docs)]
    WordBuilder,
    /// Build a Sentence
    SentenceBuilder,
    #[allow(missing_docs)]
    Matching,
    #[allow(missing_docs)]
    DressUp,
    /// Build a Scene
    SceneBuilder,
}

impl Default for Mode {
    fn default() -> Self {
        Self::SettingTable
    }
}

impl ModeExt for Mode {
    fn get_list() -> Vec<Self> {
        vec![
            Self::SettingTable,
            Self::Sorting,
            Self::WordBuilder,
            Self::SentenceBuilder,
            Self::Matching,
            Self::DressUp,
            Self::SceneBuilder,
        ]
    }

    fn as_str_id(&self) -> &'static str {
        match self {
            Self::SettingTable => "setting-table",
            Self::Sorting => "sorting",
            Self::WordBuilder => "word-builder",
            Self::SentenceBuilder => "sentence-builder",
            Self::Matching => "matching",
            Self::DressUp => "dress-up",
            Self::SceneBuilder => "scene-builder",
        }
    }

    fn label(&self) -> &'static str {
        const STR_SETTING_TABLE: &'static str = "Set a Table";
        const STR_SORTING: &'static str = "Sorting";
        const STR_WORD_BUILDER: &'static str = "Build a Word";
        const STR_SENTENCE_BUILDER: &'static str = "Build a Sentence";
        const STR_MATCHING: &'static str = "Matching";
        const STR_DRESS_UP: &'static str = "Dress Up";
        const STR_SCENE_BUILDER: &'static str = "Build a Scene";

        match self {
            Self::SettingTable => STR_SETTING_TABLE,
            Self::Sorting => STR_SORTING,
            Self::WordBuilder => STR_WORD_BUILDER,
            Self::SentenceBuilder => STR_SENTENCE_BUILDER,
            Self::Matching => STR_MATCHING,
            Self::DressUp => STR_DRESS_UP,
            Self::SceneBuilder => STR_SCENE_BUILDER,
        }
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
    /// Step 5
    Five,
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
            Self::Four => Some(Self::Five),
            Self::Five => None,
        }
    }

    fn as_number(&self) -> usize {
        match self {
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
            Self::Five => 5,
        }
    }

    fn label(&self) -> &'static str {
        //TODO - localizaton
        const STR_1: &'static str = "Design";
        const STR_2: &'static str = "Content";
        const STR_3: &'static str = "Interaction";
        const STR_4: &'static str = "Settings";
        const STR_5: &'static str = "Preview";

        match self {
            Self::One => STR_1,
            Self::Two => STR_2,
            Self::Three => STR_3,
            Self::Four => STR_4,
            Self::Five => STR_5,
        }
    }

    fn get_list() -> Vec<Self> {
        vec![Self::One, Self::Two, Self::Three, Self::Four, Self::Five]
    }
    fn get_preview() -> Self {
        Self::Five
    }
}
