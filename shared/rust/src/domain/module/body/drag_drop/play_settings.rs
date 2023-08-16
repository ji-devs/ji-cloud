use mymacros::{Deserialize, Serialize};

/// Play settings
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct PlaySettings {
    /// time limit in minutes
    pub time_limit: Option<u32>,

    /// hint style
    pub hint: Hint,
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
        Self::None
    }
}
