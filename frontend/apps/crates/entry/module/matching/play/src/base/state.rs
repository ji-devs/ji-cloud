use shared::domain::{
    asset::AssetId,
    module::{
        body::{
            Background, Instructions,
            _groups::cards::{CardPair, Mode, Step},
            matching::{ModuleData as RawData, PlayerSettings},
        },
        ModuleId,
    },
};

use futures_signals::signal::{Mutable, ReadOnlyMutable};
use std::rc::Rc;

use components::module::_common::play::prelude::*;
use utils::prelude::*;

use super::game::state::Game;

pub struct Base {
    pub asset_id: AssetId,
    pub module_id: ModuleId,
    pub mode: Mode,
    pub theme_id: ThemeId,
    pub background: Option<Background>,
    pub instructions: Instructions,
    pub feedback: Instructions,
    pub feedback_signal: Mutable<Option<Instructions>>,
    pub settings: PlayerSettings,
    pub raw_pairs: Vec<CardPair>,
    pub phase: Mutable<Phase>,
    pub module_phase: Mutable<ModulePlayPhase>,
}

#[derive(Clone)]
pub enum Phase {
    Init,
    Playing(Rc<Game>),
    Ending,
}

impl Base {
    pub async fn new(init_args: InitFromRawArgs<RawData, Mode, Step>) -> Rc<Self> {
        let InitFromRawArgs {
            asset_id,
            module_id,
            asset: _,
            raw,
            theme_id,
            ..
        } = init_args;

        let content = raw.content.unwrap_ji();

        let _self = Rc::new(Self {
            asset_id,
            module_id,
            mode: content.base.mode,
            theme_id,
            background: content.base.background,
            instructions: content.base.instructions,
            feedback: content.base.feedback,
            feedback_signal: Mutable::new(None),
            settings: content.player_settings,
            raw_pairs: content.base.pairs,
            module_phase: init_args.play_phase,
            phase: Mutable::new(Phase::Init),
        });

        _self.phase.set(Phase::Playing(Game::new(_self.clone())));

        _self
    }
}

impl BaseExt for Base {
    fn get_instructions(&self) -> Option<Instructions> {
        Some(self.instructions.clone())
    }

    fn get_feedback(&self) -> ReadOnlyMutable<Option<Instructions>> {
        self.feedback_signal.read_only()
    }

    fn handle_instructions_ended(&self) {
        self.phase.set(Phase::Ending);
        self.set_play_phase(ModulePlayPhase::Ending(Some(ModuleEnding::Positive)));
    }

    fn get_timer_minutes(&self) -> Option<u32> {
        self.settings.time_limit
    }

    fn play_phase(&self) -> Mutable<ModulePlayPhase> {
        self.module_phase.clone()
    }
}
