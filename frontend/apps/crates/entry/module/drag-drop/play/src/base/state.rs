use shared::domain::jig::{Jig, JigId, module::{ModuleId, body::{Instructions, ThemeChoice, _groups::design::{Backgrounds, Sticker}, drag_drop::{Item, Mode, ModuleData as RawData, PlaySettings, Step, TargetArea}}}};
use components::{audio::mixer::AudioMixer, module::_common::play::prelude::*};
use utils::prelude::*;
use web_sys::AudioContext;
use std::rc::Rc;

pub struct Base {
    pub jig_id: JigId,
    pub module_id: ModuleId,
    pub jig: Jig,
    pub theme_id: ThemeId,
    pub audio_mixer: AudioMixer,
    pub instructions: Instructions,
    pub settings: PlaySettings,
    pub backgrounds: Backgrounds,
    pub items: Vec<Item>,
    pub target_areas: Vec<TargetArea>,
}

impl Base {

    pub async fn new(init_args: InitFromRawArgs<RawData, Mode, Step>) -> Rc<Self> {

        let InitFromRawArgs {
            jig_id,
            module_id,
            audio_mixer,
            jig,
            raw,
            theme_id,
            ..
        } = init_args;

        let content = raw.content.unwrap_ji();

        Rc::new(Self {
            jig_id,
            module_id,
            jig,
            theme_id,
            audio_mixer,
            instructions: content.instructions,
            settings: content.play_settings,
            backgrounds: content.backgrounds,
            items: content.items,
            target_areas: content.target_areas,
        })
    }
}

impl BaseExt for Base {
    fn get_instructions(&self) -> Option<Instructions> {
        Some(self.instructions.clone())
    }

    fn get_timer_minutes(&self) -> Option<u32> {
        self.settings.time_limit
    }
}
