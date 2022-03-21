//! Types for jig Modules.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use uuid::Uuid;

/// Module bodies
pub mod body;

pub use body::Body as ModuleBody;

/// Wrapper type around [`Uuid`](Uuid), represents the **unique ID** of a module.
///
/// This uniquely identifies a module. There is no other module that shares this ID.
///
/// See also [`StableOrUniqueId`] for the two ways to identify a specific module.
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
#[serde(rename_all = "camelCase")]
pub struct ModuleId(pub Uuid);

/// Wrapper type around [`Uuid`](Uuid), represents the **stable ID** of a module.
///
/// # Notes
/// This is unique for a specific [JIG data](super::JigData) copy (draft or live). In other words, the tuple
/// `(JigId, DraftOrLive, StableModuleId)` uniquely identifies a specific module.
///
/// This ID remains stable when:
/// * Publishing JIG data from draft to live
/// * Cloning a JIG through [`jig::Clone`](crate::api::endpoints::jig::Clone)
///
/// See also [`StableOrUniqueId`] for the two ways to identify a specific module.
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
#[serde(rename_all = "camelCase")]
pub struct StableModuleId(pub Uuid);

/// Which way of finding a module to use when looking it up.
///
/// # Note:
/// The mapping between `ModuleId` and the triple `(JigId, DraftOrLive, StableModuleId)` is one-to-one.
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum StableOrUniqueId {
    /// Unique ID for the module.
    Unique(ModuleId),

    /// Stable ID for the module.
    ///
    /// This is unique for a specific [JIG data](super::JigData) copy (draft or live). In other words, the tuple
    /// `(JigId, DraftOrLive, StableModuleId)` uniquely identifies a specific module.
    ///
    /// This ID remains stable when:
    /// * Publishing JIG data from draft to live with [`jig::Publish`](crate::api::endpoints::jig::Publish)
    /// * Cloning a JIG through [`jig::Clone`](crate::api::endpoints::jig::Clone)
    Stable(StableModuleId),
}

impl fmt::Display for StableOrUniqueId {
    // Format IDs into a string
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StableOrUniqueId::Unique(id) => {
                write!(f, "{}", id.0)
            }
            StableOrUniqueId::Stable(id) => {
                write!(f, "{}", id.0)
            }
        }
    }
}

impl StableOrUniqueId {
    /// Returns [`Some`] if `self` is `Self::Unique`, [`None`] otherwise.
    pub fn unique(self) -> Option<ModuleId> {
        match self {
            StableOrUniqueId::Unique(id) => Some(id),
            StableOrUniqueId::Stable(_) => None,
        }
    }

    /// Returns [`Some`] if `self` is `Self::Stable`, [`None`] otherwise.
    pub fn stable(self) -> Option<StableModuleId> {
        match self {
            StableOrUniqueId::Unique(_) => None,
            StableOrUniqueId::Stable(id) => Some(id),
        }
    }
}

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
            _ => anyhow::bail!("Invalid ModuleKind: {}", s),
        };

        Ok(res)
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

    /// The module's stable ID.
    pub stable_id: StableModuleId,

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
    /// The module's body.
    pub body: ModuleBody,
}

impl Default for ModuleCreateRequest {
    fn default() -> Self {
        ModuleCreateRequest {
            body: ModuleBody::Cover(Default::default()),
        }
    }
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
    /// Identifies the module to be updated, either through the stable or unique ID.
    pub id: StableOrUniqueId,

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
    /// Identifies the module to be updated, either through the stable or unique ID.
    pub id: StableOrUniqueId,
}

into_uuid![ModuleId, StableModuleId];
