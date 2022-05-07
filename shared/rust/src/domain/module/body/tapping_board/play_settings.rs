use serde::{Deserialize, Serialize};

/// Play settings
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct PlaySettings {
    /// hint style
    pub hint: Hint,

    /// next style
    pub next: Next,
}

/// Hint
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
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
