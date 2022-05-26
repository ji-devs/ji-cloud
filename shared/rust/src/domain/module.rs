//! Types for jig Modules.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

/// Module bodies
pub mod body;

pub use body::Body as ModuleBody;

use super::asset::AssetId;

/// Wrapper type around [`Uuid`](Uuid), represents the **unique ID** of a module.
///
/// This uniquely identifies a module. There is no other module that shares this ID.
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
#[serde(rename_all = "camelCase")]
pub struct ModuleId(pub Uuid);

/// Represents the various kinds of data a module can represent.
#[repr(i16)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
pub enum ModuleKind {
    /// This is a sort of special module, every jig has one and it can't be deleted TODO: is that so?
    Cover = 0,

    /// Flashcards
    Flashcards = 1,

    /// Matching
    Matching = 2,

    /// Memory Game
    Memory = 3,

    /// Talking Poster
    Poster = 4,

    /// Listen & Learn
    TappingBoard = 5,

    /// Tracing
    Tracing = 6,

    /// Video
    Video = 7,

    /// Deprecated, next new module should use this slot
    //VisualQuiz = 8,

    /// Quiz Game
    CardQuiz = 9,

    /// Drag & Drop
    DragDrop = 10,

    /// Legacy
    Legacy = 11,

    /// ResourceCover
    ResourceCover = 12,

    /// Find the Answer
    FindAnswer = 13,
}

impl ModuleKind {
    /// casts `self` to a string
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Cover => "cover",
            Self::ResourceCover => "resource-cover",
            Self::Flashcards => "flashcards",
            Self::Matching => "matching",
            Self::Memory => "memory",
            Self::Poster => "poster",
            Self::TappingBoard => "tapping-board",
            Self::DragDrop => "drag-drop",
            Self::Tracing => "tracing",
            Self::Video => "video",
            Self::CardQuiz => "card-quiz",
            Self::Legacy => "legacy",
            Self::FindAnswer => "find-answer",
        }
    }

    /// Whether this ModuleKind has auto-generated content
    pub fn autogenerated(&self) -> bool {
        match self {
            Self::Flashcards | Self::Matching | Self::Memory | Self::CardQuiz => true,
            _ => false,
        }
    }
}

impl FromStr for ModuleKind {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match s {
            "cover" => Self::Cover,
            "resource-cover" => Self::ResourceCover,
            "flashcards" => Self::Flashcards,
            "matching" => Self::Matching,
            "memory" => Self::Memory,
            "poster" => Self::Poster,
            "tapping-board" => Self::TappingBoard,
            "drag-drop" => Self::DragDrop,
            "tracing" => Self::Tracing,
            "video" => Self::Video,
            "card-quiz" => Self::CardQuiz,
            "legacy" => Self::Legacy,
            "find-answer" => Self::FindAnswer,
            _ => anyhow::bail!("Invalid ModuleKind: {}", s),
        };

        Ok(res)
    }
}
impl ModuleBody {
    /// Maps module content from request body
    pub fn map_module_contents(body: &Self) -> anyhow::Result<(ModuleKind, serde_json::Value)> {
        let kind = body.kind();

        let body = match body {
            Self::CardQuiz(body) => serde_json::to_value(body)?,
            Self::Cover(body) => serde_json::to_value(body)?,
            Self::ResourceCover(body) => serde_json::to_value(body)?,
            Self::DragDrop(body) => serde_json::to_value(body)?,
            Self::Flashcards(body) => serde_json::to_value(body)?,
            Self::Matching(body) => serde_json::to_value(body)?,
            Self::MemoryGame(body) => serde_json::to_value(body)?,
            Self::Poster(body) => serde_json::to_value(body)?,
            Self::TappingBoard(body) => serde_json::to_value(body)?,
            Self::Video(body) => serde_json::to_value(body)?,
            Self::FindAnswer(body) => serde_json::to_value(body)?,
            Self::Legacy(body) => serde_json::to_value(body)?,
        };

        Ok((kind, body))
    }

    /// Transforms module content from database
    pub fn transform_response_kind(
        contents: serde_json::Value,
        kind: ModuleKind,
    ) -> anyhow::Result<Self> {
        match kind {
            ModuleKind::CardQuiz => Ok(Self::CardQuiz(serde_json::from_value(contents)?)),
            ModuleKind::Cover => Ok(Self::Cover(serde_json::from_value(contents)?)),
            ModuleKind::ResourceCover => Ok(Self::ResourceCover(serde_json::from_value(contents)?)),
            ModuleKind::DragDrop => Ok(Self::DragDrop(serde_json::from_value(contents)?)),
            ModuleKind::Flashcards => Ok(Self::Flashcards(serde_json::from_value(contents)?)),
            ModuleKind::Matching => Ok(Self::Matching(serde_json::from_value(contents)?)),
            ModuleKind::Memory => Ok(Self::MemoryGame(serde_json::from_value(contents)?)),
            ModuleKind::Poster => Ok(Self::Poster(serde_json::from_value(contents)?)),
            ModuleKind::TappingBoard => Ok(Self::TappingBoard(serde_json::from_value(contents)?)),
            ModuleKind::Video => Ok(Self::Video(serde_json::from_value(contents)?)),
            ModuleKind::FindAnswer => Ok(Self::FindAnswer(serde_json::from_value(contents)?)),
            ModuleKind::Legacy => Ok(Self::Legacy(serde_json::from_value(contents)?)),

            _ => anyhow::bail!("Unimplemented response kind"),
        }
    }
}

/// Minimal information about a module.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LiteModule {
    /// The module's unique ID.
    pub id: ModuleId,

    /// Which kind of module this is.
    pub kind: ModuleKind,

    /// Whether this module is completed.
    #[serde(default)]
    pub is_complete: bool,
}

/// Over the wire representation of a module.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Module {
    /// The module's unique ID.
    pub id: ModuleId,

    /// The module's body.
    pub body: ModuleBody,

    /// Whether the module is complete or not.
    pub is_complete: bool,

    /// Whether a jig has been updated.
    pub is_updated: bool,

    /// When the module was originally created.
    pub created_at: DateTime<Utc>,

    /// When the module was last updated.
    pub updated_at: DateTime<Utc>,
}

/// Request to create a new `Module`.
#[derive(Serialize, Deserialize, Debug)]
pub struct ModuleCreateRequest {
    /// ID for Course or JIG
    #[serde(flatten)]
    pub parent_id: AssetId,

    /// The module's body.
    pub body: ModuleBody,
}

/// Search for Live module.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModuleLiveQuery {
    /// The query string.
    #[serde(flatten)]
    pub parent_id: AssetId,
}

/// Search for Draft module.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModuleDraftQuery {
    /// The query string.
    #[serde(flatten)]
    pub parent_id: AssetId,
}

/// Response for successfully finding a module
#[derive(Serialize, Deserialize, Debug)]
pub struct ModuleResponse {
    /// The module we found
    pub module: Module,
}

/// Request to update a `Module`.
/// note: fields here cannot be nulled out (`None` means "don't change").
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModuleUpdateRequest {
    /// ID for Course or JIG
    #[serde(flatten)]
    pub parent_id: AssetId,

    /// The module's body.
    #[serde(default)]
    pub body: Option<ModuleBody>,

    /// Where to move this module to in the parent. Relevant for the order that the modules
    /// are returned when fetching JIG data.
    ///
    /// Numbers larger than the parent JIG's module count will move it to the *end*.
    #[serde(default)]
    pub index: Option<u16>,

    /// Whether the module is complete or not.
    #[serde(default)]
    pub is_complete: Option<bool>,
}

/// Request to delete a `Module`.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModuleDeleteRequest {
    /// ID for Course or JIG
    #[serde(flatten)]
    pub parent_id: AssetId,
}

into_uuid![ModuleId];
