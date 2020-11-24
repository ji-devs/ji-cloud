//! Types for jig Modules.

use std::str::FromStr;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Wrapper type around [`Uuid`], represents the ID of a module.
///
/// [`Uuid`]: ../../uuid/struct.Uuid.html
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
pub struct ModuleId(pub Uuid);

/// Represents the various kinds of data a module can represent.
#[repr(i16)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
pub enum ModuleKind {
    /// The module represents a Poster.
    Poster = 0,

    /// The module represents a Memory Game.
    MemoryGame = 1,

    /// The module represents the first / last page of a jig.
    DesignPage = 2,
}

impl ModuleKind {
    /// Converts `self` to a string
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
pub struct LiteModule {
    /// The module's ID.
    pub id: ModuleId,

    /// Which kind of module this is.
    pub kind: Option<ModuleKind>,
}

/// Over the wire representation of a module.
#[derive(Serialize, Deserialize, Debug)]
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
pub struct CreateRequest {
    /// Which kind of module this is.
    pub kind: Option<ModuleKind>,

    /// The module's json contents.
    pub body: Option<serde_json::Value>,
}

/// Response for successfully finding a module
#[derive(Serialize, Deserialize, Debug)]
pub struct GetResponse {
    /// The module we found
    pub module: Module,
}

/// Request to update a `Module`.
/// note: fields here cannot be nulled out (`None` means "don't change").
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct UpdateRequest {
    /// Which kind of module this is.
    pub kind: Option<ModuleKind>,

    /// The module's json contents.
    pub body: Option<serde_json::Value>,
}

into_uuid![ModuleId];
