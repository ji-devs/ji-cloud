use shared::domain::jig::{
    JigId, 
    Jig, 
    module::{
        ModuleId, 
        body::{
            ThemeChoice,
            Background,
            Instructions,
            _groups::cards::{Mode, Step, CardPair},
            matching::{ModuleData as RawData, Content as RawContent, PlayerSettings}, 
        }
    }
};

use futures_signals::{
    map_ref,
    signal::{self, Signal, SignalExt, Mutable},
    signal_vec::{SignalVec, SignalVecExt, MutableVec},
};
use std::{
    rc::Rc,
    cell::RefCell
};
use rand::prelude::*;
use components::module::{
    _common::play::prelude::*,
    _groups::cards::lookup::Side
};
use utils::prelude::*;
use std::future::Future;
use futures::future::join_all;
use gloo_timers::future::TimeoutFuture;
use components::audio::mixer::AudioMixer;
use super::game::state::Game;

pub struct Base {
    pub jig_id: JigId,
    pub module_id: ModuleId,
    pub mode: Mode,
    pub theme_id: ThemeId,
    pub background: Option<Background>,
    pub instructions: Instructions,
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
            jig_id,
            module_id,
            jig,
            raw,
            theme_id,
            ..
        } = init_args;

        let content = raw.content.unwrap_ji();

        let _self = Rc::new(Self {
            jig_id,
            module_id,
            mode: content.base.mode,
            theme_id,
            background: content.base.background,
            instructions: content.base.instructions, 
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

    fn get_timer_minutes(&self) -> Option<u32> {
        self.settings.time_limit
    }

    fn play_phase(&self) -> Mutable<ModulePlayPhase> {
        self.module_phase.clone()
    }
}
