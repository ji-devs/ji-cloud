use crate::domain::module::{
    body::{
        Body, BodyConvert, BodyExt, ModeExt, StepExt, ThemeId,
        _groups::design::{BaseContent, Trace},
    },
    ModuleKind,
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::convert::TryFrom;

mod play_settings;
pub use play_settings::*;

/// The body for [`FindAnswer`](crate::domain::jig::module::ModuleKind::FindAnswer) modules.
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct ModuleData {
    /// The content
    pub content: Option<Content>,
}

impl BodyExt<Mode, Step> for ModuleData {
    fn as_body(&self) -> Body {
        Body::FindAnswer(self.clone())
    }

    fn is_complete(&self) -> bool {
        self.content.is_some()
    }

    fn kind() -> ModuleKind {
        ModuleKind::FindAnswer
    }

    fn new_with_mode_and_theme(mode: Mode, theme: ThemeId) -> Self {
        ModuleData {
            content: Some(Content {
                mode,
                base: BaseContent {
                    theme,
                    ..Default::default()
                },
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
            content.base.theme = theme_id;
        }
    }

    fn get_theme(&self) -> Option<ThemeId> {
        self.content.as_ref().map(|content| content.base.theme)
    }
}

impl BodyConvert for ModuleData {}

impl TryFrom<Body> for ModuleData {
    type Error = &'static str;

    fn try_from(body: Body) -> Result<Self, Self::Error> {
        match body {
            Body::FindAnswer(data) => Ok(data),
            _ => Err("cannot convert body to Listen & Learn!"),
        }
    }
}

/// The body for [`FindAnswer`](crate::domain::jig::module::ModuleKind::FindAnswer) modules.
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct Content {
    /// The base content for all design modules
    pub base: BaseContent,

    /// The editor state
    pub editor_state: EditorState,

    /// The mode
    pub mode: Mode,

    /// Traces
    pub traces: Vec<Trace>,

    /// play settings
    pub play_settings: PlaySettings,
}

/// Editor state
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct EditorState {
    /// the current step
    pub step: Step,

    /// the completed steps
    pub steps_completed: HashSet<Step>,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
/// The mode
pub enum Mode {
    /// Words mode
    Words,
    /// Images mode
    Images,
    /// Talk mode
    Talk,
    /// Read mode
    Read,
    /// Scene mode
    Scene,
    /// Photo album mode
    PhotoAlbum,
    /// Comic mode
    Comic,
    /// Timeline mode
    Timeline,
    /// Family Tree mode
    FamilyTree,
}

impl Default for Mode {
    fn default() -> Self {
        Self::Words
    }
}

impl ModeExt for Mode {
    fn get_list() -> Vec<Self> {
        vec![
            Self::Words,
            Self::Images,
            Self::Talk,
            Self::Read,
            Self::Scene,
            Self::PhotoAlbum,
            Self::Comic,
            Self::Timeline,
            Self::FamilyTree,
        ]
    }

    fn as_str_id(&self) -> &'static str {
        match self {
            Self::Words => "words",
            Self::Images => "images",
            Self::Talk => "talk",
            Self::Read => "read",
            Self::Scene => "scene",
            Self::PhotoAlbum => "photo-album",
            Self::Comic => "comic",
            Self::Timeline => "timeline",
            Self::FamilyTree => "family-tree",
        }
    }

    fn label(&self) -> &'static str {
        const STR_WORDS_LABEL: &'static str = "Words";
        const STR_IMAGES_LABEL: &'static str = "Images";
        const STR_TALK_LABEL: &'static str = "Talking Pictures";
        const STR_READ_LABEL: &'static str = "Read Along";
        const STR_SCENE_LABEL: &'static str = "Scene";
        const STR_PHOTO_ALBUM_LABEL: &'static str = "Photo Album";
        const STR_COMIC_LABEL: &'static str = "Comics";
        const STR_TIMELINE_LABEL: &'static str = "Timeline";
        const STR_FAMILY_TREE_LABEL: &'static str = "Family Tree";

        match self {
            Self::Words => STR_WORDS_LABEL,
            Self::Images => STR_IMAGES_LABEL,
            Self::Talk => STR_TALK_LABEL,
            Self::Read => STR_READ_LABEL,
            Self::Scene => STR_SCENE_LABEL,
            Self::PhotoAlbum => STR_PHOTO_ALBUM_LABEL,
            Self::Comic => STR_COMIC_LABEL,
            Self::Timeline => STR_TIMELINE_LABEL,
            Self::FamilyTree => STR_FAMILY_TREE_LABEL,
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
        const STR_BACKGROUND: &'static str = "Design";
        const STR_CONTENT: &'static str = "Content";
        const STR_INTERACTION: &'static str = "Interaction";
        const STR_SETTINGS: &'static str = "Settings";
        const STR_PREVIEW: &'static str = "Preview";
        match self {
            Self::One => STR_BACKGROUND,
            Self::Two => STR_CONTENT,
            Self::Three => STR_INTERACTION,
            Self::Four => STR_SETTINGS,
            Self::Five => STR_PREVIEW,
        }
    }

    fn get_list() -> Vec<Self> {
        vec![Self::One, Self::Two, Self::Three, Self::Four, Self::Five]
    }
    fn get_preview() -> Self {
        Self::Five
    }
}
