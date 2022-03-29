use super::state::*;

use std::sync::atomic::Ordering;

use components::{
    audio::mixer::{AudioMixer, AudioPath, AudioSourceExt, AUDIO_MIXER},
    module::_common::play::prelude::*,
    module::_groups::cards::play::card::dom::FLIPPED_AUDIO_EFFECT,
};

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
                    let current = state.current.get_cloned();
                    if let Some(current) = current {
                        let card_id = current.others.iter().find(|card_id| card_id.pair_id == pair_id);
                        if let Some(card_id) = card_id {
                            let play_feedback = |mixer: &AudioMixer| {
                                let audio_path: AudioPath<'_> = mixer.get_random_positive().into();

                                mixer.play_oneshot(audio_path);
                            };

                            // Play the card audio first if it exists and then the feedback effect.
                            if let Some(audio) = &card_id.card.audio {
                                AUDIO_MIXER.with(move |mixer| {
                                    mixer.play_oneshot_on_ended(audio.as_source(), move || {
                                        AUDIO_MIXER.with(play_feedback);
                                    })
                                });
                            } else {
                                AUDIO_MIXER.with(play_feedback);
                            }
                        }
                    }

                    phase.set(CurrentPhase::Correct(pair_id));
                    TimeoutFuture::new(crate::config::SUCCESS_TIME).await;
                    Self::next(state);
                } else {
                    AUDIO_MIXER.with(|mixer| {
                        let audio_path = AudioPath::new_cdn(FLIPPED_AUDIO_EFFECT.to_string());

                        // Play the negative effect and then the card audio
                        mixer.play_oneshot_on_ended(audio_path, clone!(state => move || {
                            let current = state.current.get_cloned();
                            if let Some(current) = current {
                                let card_id = current.others.iter().find(|card_id| card_id.pair_id == pair_id);
                                if let Some(card_id) = card_id {
                                    if let Some(audio) = &card_id.card.audio {
                                        AUDIO_MIXER.with(|mixer| {
                                            mixer.play_oneshot(audio.as_source())
                                        });
                                    }
                                }
                            }
                        }))
                    });
                    phase.set(CurrentPhase::Wrong(pair_id));
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
