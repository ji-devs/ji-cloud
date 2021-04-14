use std::borrow::Cow;

#[cfg(feature = "backend")]
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

/// Memory Game
#[allow(missing_docs)]
pub mod memory;

/// Body kinds for Modules.
#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum Body {
    /// Module is a memory game, and has a memory game's body.
    MemoryGame(memory::ModuleData),

    /// Module kind is "unknown" - We don't have a special case for this module.
    Unknown {
        /// The module's body.
        body: serde_json::Value,
        /// The module's kind.
        kind: super::ModuleKind,
    },
}

impl Body {
    /// Gets this body's related [`ModuleKind`](super::ModuleKind)
    pub fn kind(&self) -> super::ModuleKind {
        match self {
            Self::MemoryGame(_) => super::ModuleKind::Memory,
            Self::Unknown { kind, .. } => *kind,
        }
    }

    /// Attempts to serialize `self`'s body to a `serde_json::Value`
    pub fn body_to_json(&self) -> serde_json::Result<Cow<'_, serde_json::Value>> {
        match self {
            Body::MemoryGame(body) => serde_json::to_value(body).map(Cow::Owned),
            Body::Unknown { body, .. } => Ok(Cow::Borrowed(body)),
        }
    }
}

/// Specialized version of `Body` that includes `Cover` as nullable.
///
/// Note: This will likely be removed as soon as possible.
#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum BodyResponse {
    /// Module is a memory game, and has a memory game's body.
    MemoryGame(memory::ModuleData),

    /// Module kind is a [`Cover`](super::ModuleKind::Cover).
    ///
    /// This is special cased, and the whole reason this enum exists,
    /// because `Cover` can have a `None` body, but *only* when auto-generated.
    Cover(Option<serde_json::Value>),

    /// Module kind is "unknown" - We don't have a special case for this module.
    Unknown {
        /// The module's body.
        body: serde_json::Value,
        /// The module's kind.
        kind: super::ModuleKind,
    },
}

impl BodyResponse {
    /// Gets this body's related [`ModuleKind`](super::ModuleKind)
    pub fn kind(&self) -> super::ModuleKind {
        match self {
            Self::MemoryGame(_) => super::ModuleKind::Memory,
            Self::Cover(_) => super::ModuleKind::Cover,
            Self::Unknown { kind, .. } => *kind,
        }
    }
}

