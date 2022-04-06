use std::rc::Rc;

use super::state::*;
use shared::domain::jig::module::body::_groups::design::Trace;
use utils::{drag::Drag, prelude::*, resize::get_resize_info};

use dominator::clone;

use crate::debug::*;
use components::audio::mixer::{AudioPath, AudioSourceExt, AUDIO_MIXER};
use components::collision::stickers_traces::pixels::{debug_render_hit_trace, get_hit_index};
use components::instructions::player::InstructionsPlayer;
use wasm_bindgen_futures::spawn_local;

use components::module::_common::play::prelude::*;
use shared::domain::jig::module::body::drag_drop::Next;

impl PlayState {
    pub async fn set_targets(&self) {
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

            if let Some(index) = get_hit_index(hit_source, &traces).await {
                *item.target_index.borrow_mut() = Some(index);
                log::info!("got hit! {}", index);
            }
        }
    }

    pub fn evaluate_all_completed(state: Rc<Self>) -> bool {
        let all_completed = state
            .items
            .iter()
            .filter_map(|item| {
                match item {
                    PlayItem::Interactive(item) => {
                        // Only return items which are interactive _and_ have a target trace so
                        // that we can end the game correctly when there are items which aren't
                        // meant to be placed anywhere.
                        item.target_index.borrow().as_ref().map(|_| item.clone())
                    }
                    _ => None,
                }
            })
            .all(|item| item.completed.get());

        if all_completed {
            state.feedback_player.set(Some(
                InstructionsPlayer::new(
                    state.game.base.feedback.clone(),
                    Some(clone!(state => move || {
                        match state.game.base.settings.next {
                            Next::PlaceAll => {
                                state.game.base.set_play_phase(ModulePlayPhase::Ending(Some(ModuleEnding::Next)));
                            },
                            _ => {
                                log::info!("game finished but settings is click to continue");
                            }
                        }
                    }))
                )
            ));
        }
        all_completed
    }

    pub fn evaluate(state: Rc<Self>, item: Rc<InteractiveItem>) {
        spawn_local(async move {
            let mut move_back = true;
            let mut is_correct = false;
            if let Some(target_index) = item.target_index.borrow().as_ref() {
                let target_index = *target_index;

                if let Some(hit_source) =
                    item.get_hit_source(Some(SourceTransformOverride::Current))
                {
                    let traces: Vec<&Trace> = state
                        .game
                        .base
                        .target_areas
                        .iter()
                        .map(|area| &area.trace)
                        .collect();

                    if let Some(index) = get_hit_index(hit_source, &traces).await {
                        if DEBUGGING_EVALUATION_RESULT
                            && (!DEBUGGING_EVALUATION_RESULT_ONLY_MATCH || index == target_index)
                        {
                            debug_render_hit_trace(index, &traces);
                        }
                        if index == target_index {
                            log::info!("GOT A WINNER!");
                            is_correct = true;
                        }
                    }
                }

                move_back = !is_correct;
            }

            if move_back {
                item.move_back_to_origin();
            }

            if is_correct {
                item.completed.set_neq(true);
                if !Self::evaluate_all_completed(state.clone()) {
                    item.play_audio_effect(AudioEffect::Correct);
                }
            } else {
                item.play_audio_effect(AudioEffect::Wrong);
            }
        });
    }
}

pub enum AudioEffect {
    Correct,
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
            *self.audio_user_handle.borrow_mut() =
                Some(AUDIO_MIXER.with(|mixer| mixer.play(audio.as_source(), false)));
        }
    }
    pub fn play_audio_effect(&self, effect: AudioEffect) {
        *self.audio_effect_handle.borrow_mut() =
            Some(AUDIO_MIXER.with(|mixer| mixer.play(effect.as_path(), false)));
    }

    pub fn stop_all_audio(&self) {
        *self.audio_user_handle.borrow_mut() = None;
        *self.audio_effect_handle.borrow_mut() = None;
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
        self.stop_all_audio();

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
