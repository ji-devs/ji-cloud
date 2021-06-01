use crate::domain::jig::module::{ModuleKind, body::{BodyExt, ThemeChoice, Audio, Body, Instructions, Sticker, Trace, Backgrounds, ThemeId}};
use std::convert::TryFrom;
#[cfg(feature = "backend")]
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

/// Play settings
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct PlaySettings {
    /// hint style
    pub hint: Hint,

    /// next style
    pub next: Next
}



/// Hint
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub enum Hint {
    /// None 
    None,

    /// Highlight
    Highlight
}

impl Default for Hint {
    fn default() -> Self {
        Self::Highlight
    }
}

/// Next 
#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub enum Next {
    /// Continue 
    Continue,

    /// SelectAll 
    SelectAll,

    /// Select Some
    SelectSome(usize),
}

impl Default for Next {
    fn default() -> Self {
        Self::Continue
    }
}
