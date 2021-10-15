use components::module::_common::play::prelude::*;
use shared::domain::jig::{
    module::{
        body::{legacy::ModuleData as RawData, Instructions},
        ModuleId,
    },
    JigData, JigId,
};
use utils::prelude::*;

use futures_signals::signal::Mutable;
use std::rc::Rc;

pub struct Base {
    pub jig_id: JigId,
    pub module_id: ModuleId,
    pub jig: JigData,
    pub theme_id: ThemeId,
    pub module_phase: Mutable<ModulePlayPhase>,
    pub raw: RawData,
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
            raw,
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
