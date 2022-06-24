use components::module::_common::play::prelude::*;
use shared::domain::{
    asset::{Asset, AssetId},
    module::{
        body::{
            _groups::design::{Backgrounds, Sticker},
            cover::{ModuleData as RawData, Step},
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
    pub backgrounds: Backgrounds,
    pub stickers: Vec<Sticker>,
    pub module_phase: Mutable<ModulePlayPhase>,
}

impl Base {
    pub async fn new(init_args: InitFromRawArgs<RawData, (), Step>) -> Rc<Self> {
        let InitFromRawArgs {
            asset_id,
            module_id,
            asset,
            raw,
            theme_id,
            ..
        } = init_args;

        let content = raw.content.unwrap_ji();
        let base_content = content.base;

        Rc::new(Self {
            asset_id,
            module_id,
            asset,
            theme_id,
            instructions: base_content.instructions,
            backgrounds: base_content.backgrounds,
            stickers: base_content.stickers,
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
