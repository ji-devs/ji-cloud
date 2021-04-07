#![feature(type_alias_impl_trait)]
#![feature(min_type_alias_impl_trait)]
use futures_signals::{
    map_ref,
    signal::{Mutable, ReadOnlyMutable,  SignalExt, Signal},
    signal_vec::{MutableVec, SignalVecExt, SignalVec},
    CancelableFutureHandle, 
};
use dominator::{DomBuilder, Dom, html, events, clone, apply_methods, with_node};
use wasm_bindgen::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use crate::debug;
use super::{actions, history::{self, History}, raw};
use itertools::Itertools;
use std::fmt::Write;
use serde::Deserialize;
use components::module::page::ModulePageKind;
use std::collections::HashSet;
use components::module::history::state::HistoryState;
use shared::{domain::{
    jig::{JigId, ModuleId},
    audio::AudioId
}, media::MediaLibrary};
use dominator_helpers::futures::AsyncLoader;
pub use super::card::state::*;
use wasm_bindgen_futures::spawn_local;
//See: https://users.rust-lang.org/t/eli5-existential/57780/16?u=dakom
type HistoryChangeFn = impl Fn(Option<History>);
type StateHistory = HistoryState<History, HistoryChangeFn>;
pub struct State {
    pub jig_id: JigId,
    pub module_id: ModuleId,
    pub step: Mutable<Step>,
    pub game_mode: Mutable<Option<GameMode>>,
    pub pairs: MutableVec<(Card, Card)>,
    pub steps_completed: Mutable<HashSet<Step>>,
    pub theme: Mutable<String>,
    pub instructions: Instructions,
    pub history: Rc<StateHistory>,
    pub save_loader: Rc<AsyncLoader>,
}

pub struct Instructions {
    pub audio_id: Mutable<Option<AudioId>>,
    pub text: Mutable<Option<String>>
}

impl Instructions {
    pub fn new(raw_data: Option<&raw::GameData>) -> Self {
        Self {
            audio_id: Mutable::new(None),
            text: Mutable::new(None),
        }
    }
}

impl State {
    pub fn new(jig_id: JigId, module_id: ModuleId, raw_data:Option<raw::GameData>) -> Rc<Self> {

        let game_mode:Option<GameMode> = raw_data.as_ref().map(|data| data.mode.clone().into());

        let (pairs, theme) = {
            if let Some(raw_data) = &raw_data {
                let pairs:Vec<(Card, Card)> = raw_data.pairs
                    .iter()
                    .map(|(left, right)| {
                        (left.clone().into(), right.clone().into())
                    })
                    .collect();

                (pairs, raw_data.theme.clone())
            } else {
                (
                    Vec::new(),
                    crate::config::get_themes_cloned()[0].clone()
                )
            }
        };

        let instructions = Instructions::new(raw_data.as_ref());

        let is_empty = pairs.is_empty();

        let step = Mutable::new(match debug::settings().step.as_ref() {
            Some(step) => step.clone(),
            None => Step::One
        });

        let save_loader = Rc::new(AsyncLoader::new());
        let history = Rc::new(HistoryState::new(
            History::new(raw_data),
            Self::on_history_change(save_loader.clone(), module_id.clone()),
        ));
        let _self = Rc::new(Self {
            jig_id,
            module_id,
            game_mode: Mutable::new(game_mode),
            pairs: MutableVec::new_with_values(pairs),
            step,
            steps_completed: Mutable::new(HashSet::new()),
            theme: Mutable::new(theme),
            history,
            save_loader,
            instructions
        });


        _self
    }

    //See: https://users.rust-lang.org/t/eli5-existential/57780/16?u=dakom
    fn on_history_change(save_loader: Rc<AsyncLoader>, module_id: ModuleId) -> HistoryChangeFn {
        move |history| {
            actions::save(save_loader.clone(), module_id, history.and_then(|history| history.game_data));
        }
    }


    pub fn page_kind_signal(&self) -> impl Signal<Item = ModulePageKind> {
        map_ref! {
            let has_mode = self.game_mode.signal_ref(|mode| mode.is_some()),
            let step = self.step.signal()
            => {
                if *has_mode {
                    if *step == Step::Four {
                        ModulePageKind::GridResizePreview
                    } else {
                        ModulePageKind::GridResizeScrollable
                    }
                } else {
                    ModulePageKind::GridPlain
                }
            }
        }
    }

    pub fn pairs_len_signal(&self) -> impl Signal<Item = usize> {
        self.pairs.signal_vec_cloned().len()
    }

    pub fn is_empty_signal(&self) -> impl Signal<Item = bool> {
        self.pairs_len_signal()
            .map(|len| len <= 0)
            .dedupe()
    }



}

pub type GameMode = raw::Mode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Step {
    One,
    Two,
    Three,
    Four
}

impl Step {
    pub fn label(&self) -> &'static str {
        match self {
            Step::One => crate::strings::steps_nav::STR_CONTENT,
            Step::Two => crate::strings::steps_nav::STR_DESIGN,
            Step::Three => crate::strings::steps_nav::STR_SETTINGS,
            Step::Four => crate::strings::steps_nav::STR_PREVIEW,
        }
    }

    pub fn number(&self) -> u8 {
        match self {
            Step::One => 1, 
            Step::Two => 2, 
            Step::Three => 3, 
            Step::Four => 4 
        }
    }
}


#[derive(Copy, Clone, Debug)]
pub enum Side {
    Left,
    Right,
}

impl Side {
    pub fn slot_name(&self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Right => "right",
        }
    }
}
