use components::module::_common::play::prelude::*;
use shared::domain::{
    asset::{Asset, AssetId},
    module::{
        body::{
            _groups::design::{Backgrounds, Sticker, Trace},
            find_answer::{Mode, ModuleData as RawData, PlaySettings, Step},
            Instructions,
        },
        ModuleId,
    },
};
use utils::prelude::*;

use futures_signals::signal::Mutable;
use std::rc::Rc;

pub struct Base {
    pub asset_id: AssetId,
    pub module_id: ModuleId,
    pub asset: Asset,
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
            asset_id,
            module_id,
            asset,
            raw,
            theme_id,
            ..
        } = init_args;

        let content = raw.content.unwrap_ji();

        Rc::new(Self {
            asset_id,
            module_id,
            asset,
            theme_id,
            instructions: content.base.instructions,
            settings: content.play_settings,
            backgrounds: content.base.backgrounds,
            stickers: content.base.stickers,
            traces: vec![], // TODO content.traces,
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
