use crate::domain::jig::module::{
    body::{
        Audio, Body, BodyExt, Instructions, ModeExt, StepExt, ThemeChoice,
        _groups::design::{Backgrounds, Sticker, Trace},
    },
    ModuleKind,
};
#[cfg(feature = "backend")]
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::convert::TryFrom;
use uuid::Uuid;

mod play_settings;
pub use play_settings::*;

/// The body for [`DragDrop`](crate::domain::jig::module::ModuleKind::DragDrop) modules.
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
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

    fn get_theme(&self) -> Option<ThemeChoice> {
        self.content.as_ref().map(|content| content.theme)
    }
}

impl TryFrom<Body> for ModuleData {
    type Error = &'static str;

    fn try_from(body: Body) -> Result<Self, Self::Error> {
        match body {
            Body::DragDrop(data) => Ok(data),
            _ => Err("cannot convert body to drag and drop!"),
        }
    }
}

/// The body for [`DragDrop`](crate::domain::jig::module::ModuleKind::DragDrop) modules.
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct Content {
    /// The instructions for the module.
    pub instructions: Instructions,

    /// The module's theme.
    pub theme: ThemeChoice,

    /// Backgrounds
    pub backgrounds: Backgrounds,

    /// Items (wrapper around Sticker and metadata)
    pub items: Vec<Item>,

    /// The editor state
    pub editor_state: EditorState,

    /// The mode
    pub mode: Mode,

    /// target areas
    pub target_areas: Vec<TargetArea>,

    /// play settings
    pub play_settings: PlaySettings,
}

/// Editor state
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct EditorState {
    /// the current step
    pub step: Step,

    /// the completed steps
    pub steps_completed: HashSet<Step>,
}

/// drag and drop sticker w/ metadata
#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct Item {
    /// the sticker
    pub sticker: Sticker,

    /// the kind
    pub kind: ItemKind,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
/// The mode
pub enum ItemKind {
    /// Just part of the scene
    Static,
    /// One of the draggables
    Interactive(Interactive),
}

/// drag and drop sticker w/ metadata
#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct Interactive {
    /// audio
    pub audio: Option<Audio>,

    /// target trace id
    pub target_id: Option<Uuid>,
}

/// drag and drop trace w/ metadata
#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct TargetArea {
    /// the trace
    pub trace: Trace,

    /// unique id for trace
    pub id: Uuid,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
/// The mode
pub enum Mode {
    #[allow(missing_docs)]
    SettingTable,
    #[allow(missing_docs)]
    Sorting,
    #[allow(missing_docs)]
    WordBuilder,
    #[allow(missing_docs)]
    Matching,
    #[allow(missing_docs)]
    DressUp,
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
            Self::Matching,
            Self::DressUp,
        ]
    }

    fn as_str_id(&self) -> &'static str {
        match self {
            Self::SettingTable => "setting-table",
            Self::Sorting => "sorting",
            Self::WordBuilder => "word-builder",
            Self::Matching => "matching",
            Self::DressUp => "dress-up",
        }
    }

    fn label(&self) -> &'static str {
        const STR_SETTING_TABLE: &'static str = "Setting a table";
        const STR_SORTING: &'static str = "Sorting";
        const STR_WORD_BUILDER: &'static str = "Word builder";
        const STR_MATCHING: &'static str = "Matching";
        const STR_DRESS_UP: &'static str = "Dress-up";

        match self {
            Self::SettingTable => STR_SETTING_TABLE,
            Self::Sorting => STR_SORTING,
            Self::WordBuilder => STR_WORD_BUILDER,
            Self::Matching => STR_MATCHING,
            Self::DressUp => STR_DRESS_UP,
        }
    }
}

/// The Steps
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
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
    /// Step 6
    Six,
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
            Self::Five => Some(Self::Six),
            Self::Six => None,
        }
    }

    fn as_number(&self) -> usize {
        match self {
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
            Self::Five => 5,
            Self::Six => 6,
        }
    }

    fn label(&self) -> &'static str {
        //TODO - localizaton
        const STR_1: &'static str = "Scene";
        const STR_2: &'static str = "Drag Start";
        const STR_3: &'static str = "Drop Areas";
        const STR_4: &'static str = "Drag End";
        const STR_5: &'static str = "Settings";
        const STR_6: &'static str = "Preview";

        match self {
            Self::One => STR_1,
            Self::Two => STR_2,
            Self::Three => STR_3,
            Self::Four => STR_4,
            Self::Five => STR_5,
            Self::Six => STR_6,
        }
    }

    fn get_list() -> Vec<Self> {
        vec![
            Self::One,
            Self::Two,
            Self::Three,
            Self::Four,
            Self::Five,
            Self::Six,
        ]
    }
    fn get_preview() -> Self {
        Self::Six
    }
}
