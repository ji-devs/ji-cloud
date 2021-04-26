use crate::domain::jig::module::body::{ThemeId, Instructions};
#[cfg(feature = "backend")]
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

/// The body for [`Memory`](crate::domain::jig::module::ModuleKind::Memory) modules.
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct ModuleData {
    pub instructions: Instructions,
    pub theme_id: ThemeId,
}

impl ModuleData {
    pub fn new(theme_id: ThemeId, instructions: Instructions) -> Self {
        Self {
            instructions,
            theme_id,
        }
    }
}
