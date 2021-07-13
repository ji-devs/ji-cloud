#[cfg(feature = "backend")]
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

/// Play settings
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct PlaySettings {
    /// next style
    pub next: Next,

    /// time limit in minutes
    pub time_limit: Option<u32>,

    /// hint style
    pub hint: Hint,
}

/// Hint
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub enum Hint {
    /// None
    None,

    /// Highlight
    Highlight,
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
    /// Place all
    PlaceAll,

    /// click continue
    ClickContinue,
}

impl Default for Next {
    fn default() -> Self {
        Self::PlaceAll
    }
}
