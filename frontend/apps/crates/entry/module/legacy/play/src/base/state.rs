use shared::domain::jig::{JigData, JigId, module::{ModuleId, body::{_groups::design::{Backgrounds, Sticker}, ThemeChoice, Instructions, legacy::{ModuleData as RawData}}}};
use components::{audio::mixer::AudioMixer, module::_common::play::prelude::*};
use utils::prelude::*;
use web_sys::AudioContext;
use std::rc::Rc;
use futures_signals::signal::Mutable;

pub struct Base {
    pub jig_id: JigId,
    pub module_id: ModuleId,
    pub jig: JigData,
    pub theme_id: ThemeId,
    pub module_phase: Mutable<ModulePlayPhase>,
}

impl Base {

    pub async fn new(init_args: InitFromRawArgs<RawData, (), ()>) -> Rc<Self> {

        let InitFromRawArgs {
            jig_id,
            module_id,
            jig,
            raw,
            theme_id,
            ..
        } = init_args;

        Rc::new(Self {
            jig_id,
            module_id,
            jig,
            theme_id,
            module_phase: init_args.play_phase,
        })
    }
}

impl BaseExt for Base {
    fn get_instructions(&self) -> Option<Instructions> {
        None
    }

    fn play_phase(&self) -> Mutable<ModulePlayPhase> {
        self.module_phase.clone()
    }
}
