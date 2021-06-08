use shared::domain::jig::{Jig, JigId, module::{ModuleId, body::{Backgrounds, Sticker, tapping_board::{Mode as RawMode, ModuleData as RawData, PlaySettings, TappingTrace}}}};
use components::{audio_mixer::AudioMixer, instructions::player::InstructionsPlayer, module::play::prelude::*};
use utils::prelude::*;
use web_sys::AudioContext;

pub struct Base {
    pub jig_id: JigId,
    pub module_id: ModuleId,
    pub jig: Option<Jig>,

    pub audio_mixer: AudioMixer,
    pub instructions: InstructionsPlayer,
    pub settings: PlaySettings,
    pub backgrounds: Backgrounds,
    pub stickers: Vec<Sticker>,
    pub traces: Vec<TappingTrace>,
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
            settings: content.play_settings,
            backgrounds: content.backgrounds,
            stickers: content.stickers,
            traces: content.traces,
        }
    }
}

impl BaseExt for Base {
}


/*
/// The body for [`TappingBoard`](crate::domain::jig::module::ModuleKind::TappingBoard) modules.
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
