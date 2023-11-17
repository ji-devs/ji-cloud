use crate::base::state::Phase;

use super::state::*;

use std::sync::atomic::Ordering;

use components::module::_common::play::prelude::{BaseExt, ModuleEnding, ModulePlayPhase};
use rand::prelude::*;
use shared::domain::jig::codes::JigPlaySessionModule;
use std::convert::TryInto;
use std::rc::Rc;
use utils::{prelude::*, toasts};

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
            let feedback = &state.base.feedback;
            if feedback.has_content() {
                state.base.feedback_signal.set(Some(feedback.clone()));
            } else {
                let info = state.base.play_report.lock_ref().clone();
                let info = JigPlaySessionModule::Matching(info);
                let msg = IframeAction::new(ModuleToJigPlayerMessage::AddCodeSessionInfo(info));
                if msg.try_post_message_to_player().is_err() {
                    toasts::error("Error saving progress");
                    log::info!("Error saving progress");
                }

                state.base.phase.set(Phase::Ending);
                state
                    .base
                    .set_play_phase(ModulePlayPhase::Ending(Some(ModuleEnding::Next)));
            }
        }
    }

    pub fn reset_deck(state: Rc<Self>) {
        let mut remaining: Vec<CardPairId> = state
            .base
            .raw_pairs
            .iter()
            .enumerate()
            .map(|(index, pair)| CardPairId(pair.0.clone(), pair.1.clone(), index))
            .collect();

        remaining.shuffle(&mut *state.rng.borrow_mut());

        *state.used.borrow_mut() = Vec::with_capacity(remaining.len());
        *state.remaining.borrow_mut() = remaining;
        state.current.set(None);
    }
}
