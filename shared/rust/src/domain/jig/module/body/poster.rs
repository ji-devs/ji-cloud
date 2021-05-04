use crate::domain::jig::module::body::{Instructions, Sprite, ThemeId};
#[cfg(feature = "backend")]
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

/// The body for [`Poster`](crate::domain::jig::module::ModuleKind::Poster) modules.
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct ModuleData {
    /// The instructions for the module.
    pub instructions: Instructions,

    /// The ID of the module's theme.
    pub theme_id: ThemeId,

    /// Stickers
    pub stickers: Vec<Sprite>,
}
