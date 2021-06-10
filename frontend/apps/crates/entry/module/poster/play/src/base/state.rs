use shared::domain::jig::{Jig, JigId, module::{ModuleId, body::{Backgrounds, Sticker, poster::{Mode as RawMode, ModuleData as RawData}}}};
use components::{audio_mixer::AudioMixer, instructions::player::InstructionsPlayer, module::play::prelude::*};
use utils::prelude::*;
use web_sys::AudioContext;

pub struct Base {
    pub jig_id: JigId,
    pub module_id: ModuleId,
    pub jig: Option<Jig>,

    pub audio_mixer: AudioMixer,
    pub instructions: InstructionsPlayer,
    pub backgrounds: Backgrounds,
    pub stickers: Vec<Sticker>,
}

impl Base {
    pub async fn new(audio_mixer: AudioMixer, jig_id: JigId, module_id: ModuleId, jig: Option<Jig>, raw:RawData, init_source: InitSource) -> Self {
        let content = raw.content.unwrap_ji();

        Self {
            jig_id,
            module_id,
            jig,
            audio_mixer,
            instructions: InstructionsPlayer::new(content.instructions),
            backgrounds: content.backgrounds,
            stickers: content.stickers,
        }
    }
}

impl BaseExt for Base {
}


/*
/// The body for [`Poster`](crate::domain::jig::module::ModuleKind::Poster) modules.
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct Content {
    /// The mode
    pub mode: Mode,

    /// The instructions for the module.
    pub instructions: Instructions,

    /// The module's theme.
    pub theme: ThemeChoice,

    /// Backgrounds
    pub backgrounds: Backgrounds,

    /// Stickers
    pub stickers: Vec<Sticker>,

    /// Traces
    pub traces: Vec<TappingTrace>,

    /// play settings
    pub play_settings: PlaySettings,
}
*/
