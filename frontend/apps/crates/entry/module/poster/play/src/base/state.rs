use components::{audio::mixer::AudioHandle, module::_common::play::prelude::*};
use shared::domain::module::{
    body::{
        _groups::design::{Backgrounds, Sticker},
        poster::{Mode, ModuleData as RawData, PlaySettings, Step},
        Audio, ModuleAssist,
    },
    ModuleId, StableModuleId,
};
use utils::prelude::*;

use futures_signals::signal::Mutable;
use std::{cell::RefCell, rc::Rc};

pub struct Base {
    pub module_id: ModuleId,
    pub stable_module_id: StableModuleId,
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
