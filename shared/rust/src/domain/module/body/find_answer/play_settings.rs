use serde::{Deserialize, Serialize};

/// Default limit for attempts a student can make on an answer before a trace
/// is highlighted.
pub const DEFAULT_ATTEMPTS_LIMIT: u32 = 3;

/// Play settings
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PlaySettings {
    /// Question ordering
    pub ordering: Ordering,

    /// number of attempts
    pub n_attempts: Option<u32>,

    /// time limit in minutes
    pub time_limit: Option<u32>,
}

impl Default for PlaySettings {
    fn default() -> Self {
        Self {
            n_attempts: Some(DEFAULT_ATTEMPTS_LIMIT),
            ordering: Default::default(),
            time_limit: Default::default(),
        }
    }
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
        Self::InOrder
    }
}
