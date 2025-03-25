use components::module::_common::play::prelude::*;
use futures_signals::signal::Mutable;
use shared::domain::module::{
    body::{
        ModuleAssist,
        _groups::design::{Backgrounds, Sticker},
        embed::{Mode, ModuleData as RawData, Step},
    },
    ModuleId, StableModuleId,
};
use std::rc::Rc;
use utils::prelude::*;

pub struct Base {
    pub module_id: ModuleId,
    pub stable_module_id: StableModuleId,
    pub theme_id: ThemeId,
    pub instructions: ModuleAssist,
    pub backgrounds: Backgrounds,
    pub stickers: Vec<Sticker>,
    pub module_phase: Mutable<ModulePlayPhase>,
}

impl Base {
    pub async fn new(init_args: InitFromRawArgs<RawData, Mode, Step>) -> Rc<Self> {
        let InitFromRawArgs {
            module_id,
            stable_module_id,
            raw,
            theme_id,
            ..
        } = init_args;

        let content = raw.content.unwrap_ji();
        let base_content = content.base;

        Rc::new(Self {
            module_id,
            stable_module_id,
            theme_id,
            instructions: base_content.instructions,
            backgrounds: base_content.backgrounds,
            stickers: base_content.stickers,
            module_phase: init_args.play_phase,
        })
    }
}

impl BaseExt for Base {
    fn module_id(&self) -> ModuleId {
        self.module_id
    }
    fn stable_module_id(&self) -> StableModuleId {
        self.stable_module_id
    }
    fn get_module_assist(&self) -> Option<ModuleAssist> {
        Some(self.instructions.clone())
    }
    fn play_phase(&self) -> Mutable<ModulePlayPhase> {
        self.module_phase.clone()
    }
}
