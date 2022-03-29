use crate::base::state::*;

use components::{
    audio::mixer::{AudioMixer, AudioPath, AudioSourceExt, AUDIO_MIXER},
    module::_groups::cards::play::card::dom::FLIPPED_AUDIO_EFFECT,
};
use dominator::clone;
use gloo_timers::future::TimeoutFuture;
use shared::domain::jig::module::body::_groups::cards::Card;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;

pub fn card_click(state: Rc<Base>, id: usize) -> Option<(usize, usize)> {
    let flip_state = &mut *state.flip_state.lock_mut();

    match flip_state {
        FlipState::None => {
            *flip_state = FlipState::One(id);

            // Play the flipping sound effect
            AUDIO_MIXER.with(clone!(state => move |mixer| {
                mixer.play_oneshot_on_ended(
                    // Then play the cards audio clip
                    AudioPath::new_cdn(FLIPPED_AUDIO_EFFECT.to_string()),
                    move || {
                        let card_state = state.cards.iter().find(|c| c.id == id);
                        if let Some(card_state) = card_state {
                            play_card_audio(&card_state.card);
                        }
                    }
                )
            }));
            None
        }
        FlipState::One(other) => {
            let other = *other;
            if other != id {
                *flip_state = FlipState::Two(id, other);
                Some((id, other))
            } else {
                None
            }
        }
        _ => None,
    }
}
pub fn evaluate(state: Rc<Base>, id_1: usize, id_2: usize) {
    spawn_local(async move {
        let play_effect = |positive: bool| {
            let card_state = state.cards.iter().find(|c| c.id == id_1);
            if let Some(card_state) = card_state {
                let play_feedback = move |mixer: &AudioMixer| {
                    let audio_path: AudioPath<'_> = if positive {
                        mixer.get_random_positive().into()
                    } else {
                        mixer.get_random_negative().into()
                    };

                    mixer.play_oneshot(audio_path);
                };

                // Play the card audio first if it exists, and then the feedback effect
                if let Some(audio) = &card_state.card.audio {
                    AUDIO_MIXER.with(|mixer| {
                        mixer.play_oneshot_on_ended(audio.as_source(), move || {
                            AUDIO_MIXER.with(play_feedback);
                        });
                    });
                } else {
                    AUDIO_MIXER.with(play_feedback);
                }
            }
        };

        if state.pair_lookup[id_1] == id_2 {
            let mut found_pairs = state.found_pairs.borrow_mut();
            let found_pairs_index = found_pairs.len();
            found_pairs.push((id_1, id_2));
            if let Some(card) = state.cards.iter().find(|c| c.id == id_1) {
                card.found_index.set(Some(found_pairs_index));
            }
            if let Some(card) = state.cards.iter().find(|c| c.id == id_2) {
                card.found_index.set(Some(found_pairs_index));
            }

            play_effect(true);
        } else {
            play_effect(false);
            TimeoutFuture::new(2_000).await;
        }

        state.flip_state.set(FlipState::None);
    })
}

fn play_card_audio(card: &Card) {
    if let Some(audio) = &card.audio {
        AUDIO_MIXER.with(|mixer| mixer.play_oneshot(audio.as_source()));
    }
}
