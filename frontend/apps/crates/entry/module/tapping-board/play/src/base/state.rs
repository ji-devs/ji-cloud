use shared::domain::jig::{Jig, JigId, module::{ModuleId, body::{_groups::design::{Backgrounds, Sticker, Trace}, ThemeChoice, Instructions, tapping_board::{Mode, Step, ModuleData as RawData, PlaySettings, }}}};
use components::{audio::mixer::AudioMixer, module::_common::play::prelude::*};
use utils::prelude::*;
use web_sys::AudioContext;
use std::rc::Rc;
use futures_signals::signal::Mutable;

pub struct Base {
    pub jig_id: JigId,
    pub module_id: ModuleId,
    pub jig: Jig,
    pub theme_id: ThemeId,
    pub instructions: Instructions,
    pub settings: PlaySettings,
    pub backgrounds: Backgrounds,
    pub stickers: Vec<Sticker>,
    pub traces: Vec<Trace>,
    pub module_phase: Mutable<ModulePlayPhase>,
}

impl Base {

    pub async fn new(init_args: InitFromRawArgs<RawData, Mode, Step>) -> Rc<Self> {

        let InitFromRawArgs {
            jig_id,
            module_id,
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
            instructions: content.base.instructions,
            settings: content.play_settings,
            backgrounds: content.base.backgrounds,
            stickers: content.base.stickers,
            traces: content.traces,
            module_phase: init_args.play_phase,
        })
    }
}

impl BaseExt for Base {
    fn get_instructions(&self) -> Option<Instructions> {
        Some(self.instructions.clone())
    }

    fn play_phase(&self) -> Mutable<ModulePlayPhase> {
        self.module_phase.clone()
    }
}
