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

    /// Module is a [`Cover`](super::ModuleKind::Cover).
    ///
    /// This exists as an empty enum because cover *needs* to exist, but it also isn't decided yet.
    Cover,
}

impl Body {
    /// Gets this body's related [`ModuleKind`](super::ModuleKind)
    pub fn kind(&self) -> super::ModuleKind {
        match self {
            Self::Cover => super::ModuleKind::Cover,
            Self::MemoryGame(_) => super::ModuleKind::Memory,
        }
    }
}
