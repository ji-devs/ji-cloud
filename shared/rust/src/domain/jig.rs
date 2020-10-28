//! Types for JIGs.

use super::Publish;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Wrapper type around [`Uuid`], represents the ID of a JIG.
///
/// [`Uuid`]: ../../uuid/struct.Uuid.html
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
pub struct JigId(pub Uuid);

/// Wrapper type around [`Uuid`], represents the ID of a module.
///
/// [`Uuid`]: ../../uuid/struct.Uuid.html
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
pub struct ModuleId(pub Uuid);

/// Request to create a new JIG.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CreateRequest {
    /// The JIG's name.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub display_name: Option<String>,

    /// The JIG's cover module.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub cover: Option<ModuleId>,

    /// The JIG's ending module.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub ending: Option<ModuleId>,

    /// The JIG's remaining modules.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub modules: Vec<ModuleId>,

    /// When the JIG should be considered published (if at all).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub publish_at: Option<Publish>,
}

/// Response for successfully creating a JIG.
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateResponse {
    /// The ID of the newly created JIG.
    pub id: JigId,
}

/// The over-the-wire representation of a JIG.
#[derive(Serialize, Deserialize, Debug)]
pub struct Jig {
    /// The ID of the JIG.
    pub id: JigId,

    /// The JIG's name.
    pub display_name: Option<String>,

    /// The JIG's cover module.
    pub cover: LiteModule,

    /// The JIG's ending module.
    pub ending: LiteModule,

    /// The JIG's remaining modules.
    pub modules: Vec<LiteModule>,

    /// The ID of the JIG's original creator (`None` if unknown).
    pub creator_id: Option<Uuid>,

    /// The current author
    pub author_id: Option<Uuid>,

    /// When the JIG should be considered published (if at all).
    pub publish_at: Option<DateTime<Utc>>,
}

/// Represents the various kinds of data a module can represent.
#[repr(i16)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[derive(Serialize, Deserialize, Debug)]
pub enum ModuleKind {
    /// The module represents a Poster.
    Poster = 0,

    /// The module represents a Memory Game.
    MemoryGame = 1,

    /// The module represents the first / last page of a jig.
    DesignPage = 2,
}

/// Minimal information about a module.
#[derive(Serialize, Deserialize, Debug)]
pub struct LiteModule {
    /// The module's ID.
    pub id: ModuleId,

    /// Which kind of module this is.
    pub kind: Option<ModuleKind>,
}

/// The response returned when a request for `GET`ing a jig is successful.
#[derive(Serialize, Deserialize, Debug)]
pub struct GetResponse {
    /// The requested JIG.
    pub jig: Jig,
}

/// Request for updating a JIG.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct UpdateRequest {
    /// The JIG's name.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub display_name: Option<String>,

    /// The JIG's cover module.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub cover: Option<ModuleId>,

    /// The JIG's ending module.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub ending: Option<ModuleId>,

    /// The JIG's remaining modules.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub modules: Option<Vec<ModuleId>>,

    /// The current author
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub author_id: Option<Uuid>,

    /// When the JIG should be considered published (if at all).
    #[serde(deserialize_with = "super::deserialize_optional_field")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub publish_at: Option<Option<Publish>>,
}
