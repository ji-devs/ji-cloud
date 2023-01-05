use components::module::_common::play::prelude::*;
use futures_signals::signal::Mutable;
use shared::domain::{
    asset::AssetId,
    module::{
        body::{
            ModuleAssist,
            _groups::design::{Backgrounds, Sticker},
            video::{Mode, ModuleData as RawData, PlaySettings, Step},
        },
        ModuleId,
    },
};
use std::rc::Rc;
use utils::prelude::*;

pub struct Base {
    pub asset_id: AssetId,
    pub module_id: ModuleId,
    pub theme_id: ThemeId,
    pub instructions: ModuleAssist,
    pub backgrounds: Backgrounds,
    pub stickers: Vec<Sticker>,
    pub play_settings: PlaySettings,
    pub module_phase: Mutable<ModulePlayPhase>,
}

impl Base {
    pub async fn new(init_args: InitFromRawArgs<RawData, Mode, Step>) -> Rc<Self> {
        let InitFromRawArgs {
            asset_id,
            module_id,
            raw,
            theme_id,
            ..
        } = init_args;

        let content = raw.content.unwrap_ji();
        let base_content = content.base;

        Rc::new(Self {
            asset_id,
            module_id,
            theme_id,
            instructions: base_content.instructions,
            backgrounds: base_content.backgrounds,
            stickers: base_content.stickers,
            play_settings: content.play_settings,
            module_phase: init_args.play_phase,
        })
    }
}

impl BaseExt for Base {
    fn get_module_assist(&self) -> Option<ModuleAssist> {
        Some(self.instructions.clone())
    }
    fn play_phase(&self) -> Mutable<ModulePlayPhase> {
        self.module_phase.clone()
    }
}
