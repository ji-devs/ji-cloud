use components::{audio::mixer::AudioHandle, module::_common::play::prelude::*};
use shared::domain::{
    asset::{Asset, AssetId},
    module::{
        body::{
            _groups::design::{Backgrounds, Sticker},
            poster::{Mode, ModuleData as RawData, PlaySettings, Step},
            Audio, ModuleAssist,
        },
        ModuleId,
    },
};
use utils::prelude::*;

use futures_signals::signal::Mutable;
use std::{cell::RefCell, rc::Rc};

pub struct Base {
    pub asset_id: AssetId,
    pub module_id: ModuleId,
    pub asset: Asset,
    pub theme_id: ThemeId,
    pub instructions: ModuleAssist,
    pub audio: Option<Audio>,
    pub audio_handle: Rc<RefCell<Option<AudioHandle>>>,
    pub backgrounds: Backgrounds,
    pub stickers: Vec<Sticker>,
    pub module_phase: Mutable<ModulePlayPhase>,
    pub play_settings: PlaySettings,
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
        let base_content = content.base;

        Rc::new(Self {
            asset_id,
            module_id,
            asset,
            theme_id,
            instructions: base_content.instructions,
            audio: content.audio,
            audio_handle: Rc::new(RefCell::new(None)),
            backgrounds: base_content.backgrounds,
            stickers: base_content.stickers,
            module_phase: init_args.play_phase,
            play_settings: content.play_settings,
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
