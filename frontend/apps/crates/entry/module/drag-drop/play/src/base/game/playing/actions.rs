use std::rc::Rc;

use super::state::*;
use shared::domain::module::body::_groups::design::Trace;
use utils::{drag::Drag, prelude::*, resize::get_resize_info};

use crate::debug::*;
use components::audio::mixer::{AudioPath, AudioSourceExt, AUDIO_MIXER};
use components::collision::stickers_traces::pixels::{debug_render_hit_trace, get_hit_index};
use wasm_bindgen_futures::spawn_local;

impl PlayState {
    pub fn set_targets(&self) {
        let items = self.items.iter().filter_map(|item| match item {
            PlayItem::Interactive(item) => Some(item.clone()),
            _ => None,
        });

        let traces: Vec<&Trace> = self
            .game
            .base
            .target_areas
            .iter()
            .map(|area| &area.trace)
            .collect();

        for item in items {
            let hit_source = item
                .get_hit_source(Some(SourceTransformOverride::Target))
                .unwrap_ji();

            if let Some(index) = get_hit_index(hit_source, &traces) {
                *item.target_index.borrow_mut() = Some(index);
            }
        }
    }

    pub fn evaluate_all_completed(state: Rc<Self>) -> bool {
        let all_completed = state
            .items
            .iter()
            .enumerate()
            .filter_map(|(idx, item)| {
                match item {
                    PlayItem::Interactive(item) => {
                        // Only return items which are interactive _and_ have target traces so
                        // that we can end the game correctly when there are items which aren't
                        // meant to be placed anywhere.
                        // Note: We only care that a sticker has a target trace, not which one it
                        // is in. The `all()` call will ensure that all the items are completed
                        // correctly. So we look for the first target trace that matches this
                        // sticker.
                        state
                            .game
                            .base
                            .item_targets
                            .iter()
                            .find(|item_target| idx == item_target.sticker_idx)
                            .map(|_| item.completed.get())
                    }
                    _ => None,
                }
            })
            .all(|completed| completed);

        all_completed
    }

    pub fn evaluate(state: Rc<Self>, item_index: usize, item: Rc<InteractiveItem>) {
        spawn_local(async move {
            let mut is_correct = false;

            if let Some(hit_source) = item.get_hit_source(Some(SourceTransformOverride::Current)) {
                let traces: Vec<&Trace> = state
                    .game
                    .base
                    .target_areas
                    .iter()
                    .map(|area| &area.trace)
                    .collect();

                if let Some(index) = get_hit_index(hit_source, &traces) {
                    is_correct = state.game.base.item_targets.iter().any(|item_target| {
                        item_index == item_target.sticker_idx && item_target.trace_idx == index
                    });

                    if DEBUGGING_EVALUATION_RESULT
                        && (!DEBUGGING_EVALUATION_RESULT_ONLY_MATCH || is_correct)
                    {
                        debug_render_hit_trace(index, &traces);
                    }
                }
            }

            if !is_correct {
                // Move the item back to it's origin
                item.move_back_to_origin();
                item.play_audio_effect(AudioEffect::Wrong);
            } else {
                item.completed.set_neq(true);
                if !Self::evaluate_all_completed(state.clone()) {
                    item.play_audio_effect(AudioEffect::Correct);
                } else {
                    // Play JIG positive feedback sound
                    AUDIO_MIXER.with(|mixer| {
                        let positive_audio: AudioPath<'_> = mixer.get_random_positive().into();
                        mixer.play_oneshot_on_ended(positive_audio, move || {
                            // Once the positive feedback effect has played, we can show/play the
                            // feedback for the activity. If we played this at the same time, it
                            // we could have two audio clips playing simultaneously which would be
                            // noisy and distracting from the intent of the feedbacks.
                            state
                                .game
                                .base
                                .feedback_signal
                                .set(Some(state.game.base.feedback.clone()));
                        });
                    });
                }
            }
        });
    }
}

pub enum AudioEffect {
    /// Drop sound
    Correct,
    /// Sliding back sound
    Wrong,
}

impl AudioEffect {
    pub fn as_path(&self) -> AudioPath<'_> {
        let filename = match self {
            AudioEffect::Correct => "correct",
            AudioEffect::Wrong => "wrong",
        };

        AudioPath::new_cdn(format!("module/drag-drop/{}.mp3", filename))
    }
}

impl InteractiveItem {
    pub fn try_play_user_audio(&self) {
        if let Some(audio) = self.audio.as_ref() {
            AUDIO_MIXER.with(|mixer| {
                mixer.play_oneshot(audio.as_source());
            });
        }
    }
    pub fn play_audio_effect(&self, effect: AudioEffect) {
        AUDIO_MIXER.with(|mixer| {
            mixer.play_oneshot(effect.as_path());
        });
    }

    pub fn start_drag(&self, x: i32, y: i32) {
        if !self.completed.get() {
            self.try_play_user_audio();

            self.drag
                .set(Some(Rc::new(Drag::new(x, y, 0.0, 0.0, true))));
        }
    }

    pub fn try_move_drag(&self, x: i32, y: i32) {
        if let Some(drag) = self.drag.lock_ref().as_ref() {
            if let Some((_, diff)) = drag.update(x, y) {
                let resize_info = get_resize_info();
                let (diff_x, diff_y) = resize_info.get_px_normalized(diff.x as f64, diff.y as f64);

                self.curr_transform.replace_with(|t| {
                    let mut t = t.clone();
                    t.add_translation_2d(diff_x * -1.0, diff_y * -1.0);

                    t
                });
            }
        }
    }

    pub fn try_end_drag(&self, _x: i32, _y: i32) -> bool {
        if self.drag.lock_ref().is_some() {
            let _drag = self.drag.lock_mut().take().unwrap_ji();
            //self.curr_offset.set((0.0, 0.0));
            true
        } else {
            false
        }
    }

    pub fn move_back_to_origin(&self) {
        self.curr_transform.set(self.sticker.transform().clone());
    }
}
