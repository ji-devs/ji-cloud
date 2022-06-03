use serde::{Deserialize, Serialize};

/// Play settings
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct PlaySettings {
    /// Question ordering
    pub ordering: Ordering,

    /// number of attempts
    pub n_attempts: Option<u8>,

    /// time limit in minutes
    pub time_limit: Option<u32>,

    /// Next style
    pub next: Next,
}

/// Ordering of questions
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum Ordering {
    /// Questions should be randomized
    Randomize,

    /// Questions should be shown in order
    InOrder,
}

impl Default for Ordering {
    fn default() -> Self {
        Self::Randomize
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
