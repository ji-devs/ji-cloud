use crate::{
    domain::{audio::AudioId, image::ImageId, jig::module::theme::ThemeId},
    media::MediaLibrary,
};
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

#[derive(Clone, Default, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct Instructions {
    pub text: Option<String>,
    pub audio_id: Option<AudioId>,
}

impl Instructions {
    pub fn new() -> Self {
        Self {
            text: None,
            audio_id: None,
        }
    }
}


impl ModuleData {
    pub fn new(theme_id: ThemeId, instructions: Instructions) -> Self {
        Self {
            instructions,
            theme_id,
        }
    }
}
