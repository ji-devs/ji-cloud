//! Types for Jig short codes for sharing

use std::ops::Deref;

use serde::{Deserialize, Serialize};

/// Settings for the player session.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JigPlayerSettings {
    /// Text direction, left-to-right or right-to-left
    #[serde(default)]
    pub direction: TextDirection,
    /// Scoring
    #[serde(default)]
    pub scoring: bool,
    /// Whether or not to enable drag assist
    #[serde(default)]
    pub drag_assist: bool,
}

impl Default for JigPlayerSettings {
    fn default() -> Self {
        Self {
            direction: TextDirection::default(),
            scoring: false,
            drag_assist: false,
        }
    }
}

/// Sets text direction for the jig.
#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[repr(i16)]
pub enum TextDirection {
    /// left to right
    #[serde(rename = "ltr")]
    LeftToRight = 0,

    /// right to left
    #[serde(rename = "rtl")]
    RightToLeft = 1,
}

impl Default for TextDirection {
    fn default() -> Self {
        Self::LeftToRight
    }
}

impl TextDirection {
    /// check if is left to right
    pub fn is_ltr(&self) -> bool {
        self == &Self::LeftToRight
    }

    /// check if is right to left
    pub fn is_rtl(&self) -> bool {
        self == &Self::RightToLeft
    }
}

#[derive(Clone, Default, Serialize, Deserialize, Debug, Eq, PartialEq)]
/// Module config passed to the JIG player when a module starts
pub struct ModuleConfig {
    /// How player navigation should be handled
    pub navigation_handler: PlayerNavigationHandler,
    /// Optional timer to use for the module
    pub timer: Option<Seconds>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
/// How JIG player navigation should be handled
pub enum PlayerNavigationHandler {
    /// The JIG player handles the navigation
    Player,
    /// The module handles navigation
    Module,
}

impl Default for PlayerNavigationHandler {
    fn default() -> Self {
        Self::Player
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
/// Newtype for timer seconds
pub struct Seconds(pub u32);

impl Deref for Seconds {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
