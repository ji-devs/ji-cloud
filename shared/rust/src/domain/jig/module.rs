//! Types for jig Modules.

#[cfg(feature = "backend")]
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

/// Wrapper type around [`Uuid`](Uuid), represents the ID of a module.
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct ModuleId(pub Uuid);

/// Represents the various kinds of data a module can represent.
#[repr(i16)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub enum ModuleKind {
    /// The module represents a Poster.
    Poster = 0,

    /// The module represents a Memory Game.
    MemoryGame = 1,

    /// The module represents the first / last page of a jig.
    DesignPage = 2,
}

impl ModuleKind {
    /// casts `self` to a string
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Poster => "poster",
            Self::MemoryGame => "memory",
            Self::DesignPage => "design-page",
        }
    }
}

impl FromStr for ModuleKind {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match s {
            "poster" => Self::Poster,
            "memory" => Self::MemoryGame,
            "design-page" => Self::DesignPage,
            _ => anyhow::bail!("Invalid ModuleKind: {}", s),
        };

        Ok(res)
    }
}

/// Minimal information about a module.
#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct LiteModule {
    /// The module's ID.
    pub id: ModuleId,

    /// Which kind of module this is.
    pub kind: Option<ModuleKind>,
}

/// Over the wire representation of a module.
#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct Module {
    /// The module's ID.
    pub id: ModuleId,

    /// Which kind of module this is.
    pub kind: Option<ModuleKind>,

    /// The module's json contents.
    pub body: Option<serde_json::Value>,
}

/// Request to create a new `Module`.
#[derive(Serialize, Deserialize, Debug, Default)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct CreateRequest {
    /// Which kind of module this is.
    pub kind: Option<ModuleKind>,

    /// The module's json contents.
    pub body: Option<serde_json::Value>,
}

/// Response for successfully finding a module
#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct GetResponse {
    /// The module we found
    pub module: Module,
}

/// Request to update a `Module`.
/// note: fields here cannot be nulled out (`None` means "don't change").
#[derive(Serialize, Deserialize, Debug, Default)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct UpdateRequest {
    /// Which kind of module this is.
    pub kind: Option<ModuleKind>,

    /// The module's json contents.
    pub body: Option<serde_json::Value>,
}

into_uuid![ModuleId];
