#[cfg(feature = "backend")]
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

/// Theme Ids. Used in various modules
/// See the frontend extension trait for more info
#[derive(Clone, Copy, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub enum ThemeId {
    /// No theme id set (a.k.a. default)
    None,
    /// Blueish theme
    Chalkboard,
    /// Orangeish theme
    HappyBrush,
}
