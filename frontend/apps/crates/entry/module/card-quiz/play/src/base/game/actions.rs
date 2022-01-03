use super::state::*;

use std::sync::atomic::Ordering;

use components::{module::_common::play::prelude::*, audio::mixer::play_random_positive};

use crate::base::state::Phase;
use dominator::clone;
use futures_signals::signal::Mutable;
use gloo_timers::future::TimeoutFuture;
use rand::prelude::*;
use std::convert::TryInto;
use std::rc::Rc;
use utils::prelude::*;
use wasm_bindgen_futures::spawn_local;

impl Game {
    pub fn next(state: Rc<Self>) {
        let rounds_played = state.rounds_played.load(Ordering::SeqCst);

        let has_ended = {
            if rounds_played >= state.base.settings.n_rounds.try_into().unwrap_ji() {
                true
            } else if state.remaining.borrow().len() == 0 {
                log::info!("deck finished, re-shuffling!");
                Self::reset_deck(state.clone());
                false
            } else {
                false
            }
        };

        if !has_ended {
            state.current.set(Some(Current::new(state.clone())));
            state
                .rounds_played
                .store(rounds_played + 1, Ordering::SeqCst);
            log::info!(
                "playing round {} of {}",
                rounds_played + 1,
                state.base.settings.n_rounds
            );
        } else {
            log::info!("GAME OVER!");
            state.base.phase.set(Phase::Ending);
            state
                .base
                .set_play_phase(ModulePlayPhase::Ending(Some(ModuleEnding::Positive)));
        }
    }

    pub fn reset_deck(state: Rc<Self>) {
        let mut remaining: Vec<CardPairId> = state
            .base
            .raw_pairs
            .iter()
            .enumerate()
            .map(|(index, pair)| {
                CardPairId(
                    CardId {
                        card: pair.0.clone(),
                        pair_id: index,
                    },
                    CardId {
                        card: pair.1.clone(),
                        pair_id: index,
                    },
                )
            })
            .collect();

        remaining.shuffle(&mut *state.rng.borrow_mut());

        *state.used.borrow_mut() = Vec::with_capacity(remaining.len());
        *state.remaining.borrow_mut() = remaining;
        state.current.set(None);
    }

    pub fn evaluate(state: Rc<Self>, pair_id: usize, phase: Mutable<CurrentPhase>) {
        if phase.get() == CurrentPhase::Waiting {
            spawn_local(clone!(state, pair_id, phase => async move {
                if pair_id == state.current.lock_ref().as_ref().unwrap_ji().target.pair_id {
                    play_random_positive();

                    phase.set(CurrentPhase::Correct(pair_id));
                    TimeoutFuture::new(crate::config::SUCCESS_TIME).await;
                    Self::next(state);
                } else {
                    // We should be able to safely assume that current is Some(_), but
                    // double-check here anyway because assumptions are bad.
                    if let Some(current) = &*state.current.lock_ref() {
                        current.incorrect_choices.borrow_mut().push(pair_id);
                    }
                    phase.set(CurrentPhase::Waiting);
                }

            }));
        }
    }
}
