use super::state::*;
use components::{
    audio::mixer::{AudioPath, AUDIO_MIXER},
    module::{
        _common::play::prelude::{BaseExt, ModuleEnding, ModulePlayPhase},
        _groups::cards::{lookup::Side, play::card::dom::FLIPPED_AUDIO_EFFECT},
    },
};
use gloo_timers::future::TimeoutFuture;
use shared::domain::module::body::_groups::cards::CardPair;

use crate::base::state::{Base, Phase};

use dominator::clone;
use rand::prelude::*;
use std::{rc::Rc, sync::atomic::Ordering};
use utils::prelude::*;

impl Game {
    pub fn next(&self) {
        self.animation_loader.cancel();
        *self.audio_handle.borrow_mut() = None;

        // Update rounds played before anything else happens so that we can
        // be sure that it represent the actual amount of pairs the student
        // has played through.
        let rounds_played = self.rounds_played.load(Ordering::SeqCst) + 1;
        self.rounds_played.store(rounds_played, Ordering::SeqCst);

        let max_rounds = self
            .base
            .settings
            .view_pairs
            .unwrap_or_else(|| self.base.raw_pairs.len() as u32)
            .min(self.base.raw_pairs.len() as u32);

        let has_ended = rounds_played >= max_rounds as usize;

        log::info!("{:?}", self.base.settings.view_pairs);
        log::info!("{:?}", self.base.raw_pairs.len());
        log::info!("{rounds_played} >= {max_rounds}");

        if !has_ended {
            self.gate.set_neq(Gate::Waiting);

            //borrow-checker fails with if/else here
            {
                if let Some(next) = get_current(&self.base, &mut self.deck.borrow_mut()) {
                    self.current.set(next);
                    return;
                }
            }

            self.reset_deck();
        } else {
            let feedback = &self.base.feedback;
            if feedback.has_content() {
                self.base.feedback_signal.set(Some(feedback.clone()));
            } else {
                self.base.phase.set(Phase::Ending);
                self.base
                    .set_play_phase(ModulePlayPhase::Ending(Some(ModuleEnding::Next)));
            }
        }
    }

    pub fn flip(state: Rc<Self>) {
        if state.gate.get() == Gate::Waiting {
            state.gate.set(Gate::Flipping);
            let audio = state.current.get_cloned().other.audio;

            state.animation_loader.load(clone!(state => async move {
                let narration = async {
                    let ended = AUDIO_MIXER.with(|mixer| {
                        let (handle, ended) = mixer.play_until_ended(AudioPath::new_cdn(
                            FLIPPED_AUDIO_EFFECT.to_string(),
                        ));
                        *state.audio_handle.borrow_mut() = Some(handle);
                        ended
                    });

                    if ended.await.is_err() {
                        return false;
                    }

                    if let Some(audio) = audio {
                        let ended = AUDIO_MIXER.with(|mixer| {
                            let (handle, ended) = mixer.play_until_ended(audio.into());
                            *state.audio_handle.borrow_mut() = Some(handle);
                            ended
                        });

                        if ended.await.is_err() {
                            return false;
                        }
                    }

                    true
                };

                let (audio_finished, _) = futures::join!(
                    narration,
                    TimeoutFuture::new(crate::config::SHOW_TIME),
                );

                if !audio_finished {
                    return;
                }

                state.gate.set(Gate::FinishingFlip);
                TimeoutFuture::new(crate::config::FLIP_TIME).await;
                state.gate.set(Gate::Waiting);
                state.next();
            }));
        }
    }

    fn reset_deck(&self) {
        let mut rng = self.rng.borrow_mut();

        let mut deck = get_fresh_deck(&self.base, &mut rng);

        let current = get_current(&self.base, &mut deck).unwrap_ji();

        *self.deck.borrow_mut() = deck;
        self.current.set(current);
    }
}

pub(super) fn get_fresh_deck(base: &Base, rng: &mut ThreadRng) -> Vec<CardPair> {
    let mut deck = base.raw_pairs.clone();

    deck.shuffle(rng);

    deck
}

pub(super) fn get_current(base: &Base, deck: &mut Vec<CardPair>) -> Option<Current> {
    deck.pop().map(|pair| {
        let current = if base.settings.swap {
            Current {
                card: pair.0,
                other: pair.1,
                side: Side::Left,
            }
        } else {
            Current {
                card: pair.1,
                other: pair.0,
                side: Side::Right,
            }
        };

        current
    })
}
