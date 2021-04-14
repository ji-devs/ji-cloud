use serde::{Deserialize, Serialize};

/// Theme Ids. Used in various modules
/// See the frontend extension trait for more info
#[derive(Clone, Copy, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub enum ThemeId {
    /// No theme id set (a.k.a. default)
    None,
    /// Blueish theme
    Chalkboard,
    /// Orangeish theme
    HappyBrush,
}
