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
use super::{raw, history::{self, History}};
use itertools::Itertools;
use std::fmt::Write;
use serde::Deserialize;
use components::module::page::ModulePageKind;
use std::collections::HashSet;
use components::module::history::state::HistoryState;
use shared::domain::jig::{JigId, ModuleId};
pub use super::card::state::*;

pub struct State {
    pub jig_id: JigId,
    pub module_id: ModuleId,
    pub step: Mutable<Step>,
    pub game_mode: Mutable<Option<GameMode>>,
    pub pairs: MutableVec<(Card, Card)>,
    pub steps_completed: Mutable<HashSet<Step>>,
    pub theme: Mutable<String>,
    pub history: Rc<HistoryState<History>>,
}


impl State {
    pub fn new(jig_id: JigId, module_id: ModuleId, raw_data:Option<raw::GameData>) -> Self {

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

        let is_empty = pairs.is_empty();

        let step = Mutable::new(debug::settings().step.unwrap_or(Step::One));
        Self {
            jig_id,
            module_id,
            game_mode: Mutable::new(game_mode),
            pairs: MutableVec::new_with_values(pairs),
            step,
            steps_completed: Mutable::new(HashSet::new()),
            theme: Mutable::new(theme),
            history: Rc::new(HistoryState::new(History::new(raw_data))),
        }
    }

    pub fn to_save_signal(&self) -> impl Signal<Item = Option<raw::GameData>> {
        map_ref! {
            let game_mode = self.game_mode.signal(),
            let pairs = self.pairs.signal_vec_cloned().to_signal_cloned(),
            let theme = self.theme.signal_cloned()
            => {
                None
                /* TODO
                game_mode.map(|game_mode| {

                })
                */
            }
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
