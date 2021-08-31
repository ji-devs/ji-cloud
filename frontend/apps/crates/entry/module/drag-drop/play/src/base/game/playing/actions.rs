use std::rc::Rc;
use utils::math::{BoundsF64, bounds, quat, vec2};
use utils::{prelude::*, drag::Drag, resize::get_resize_info};
use super::state::*;
use shared::domain::jig::module::body::_groups::design::{Trace, Sticker};
use awsm_web::{dom::StyleExt, canvas::get_2d_context};
use web_sys::{HtmlCanvasElement};
use dominator::clone;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use std::collections::HashMap;
use components::collision::stickers_traces::pixels::{get_hit_index, StickerHitSource, StickerBoundsKind, debug_render_hit_trace};
use wasm_bindgen_futures::spawn_local;
use components::audio::mixer::AudioPath;

cfg_if::cfg_if! {
    if #[cfg(debug_assertions)] {
        const DEBUGGING_EVALUATION_RESULT:bool = false;
        const DEBUGGING_EVALUATION_RESULT_ONLY_MATCH:bool = false;
    } else {
        const DEBUGGING_EVALUATION_RESULT:bool = false;
        const DEBUGGING_EVALUATION_RESULT_ONLY_MATCH:bool = false;
    }
}


impl PlayState {
    pub async fn set_targets(&self) {

        let items = self.items 
            .iter()
            .filter(|item| {
                match item {
                    PlayItem::Interactive(_) => true,
                    _ => false
                }
            })
            .map(|item| item.get_interactive_unchecked());

        let traces:Vec<&Trace> = self.game.base.target_areas.iter().map(|area| &area.trace).collect();

        for item in items {
            let hit_source = item.get_hit_source(Some(SourceTransformOverride::Target)).unwrap_ji();

            if let Some(index) = get_hit_index(hit_source, &traces).await {
                *item.target_index.borrow_mut() = Some(index);
                log::info!("got hit! {}", index);
            }
            
        }
    }

    pub fn evaluate(&self, item: Rc<InteractiveItem>) {
        let game = self.game.clone();

        spawn_local(async move {
            let mut move_back = false;
            let mut is_correct = false;
            if let Some(target_index) = item.target_index.borrow().as_ref() {
                let target_index = *target_index;


                if let Some(hit_source) = item.get_hit_source(Some(SourceTransformOverride::Current)) {

                    let traces:Vec<&Trace> = game.base.target_areas.iter().map(|area| &area.trace).collect();

                    if let Some(index) = get_hit_index(hit_source, &traces).await {
                        if DEBUGGING_EVALUATION_RESULT {
                            if !DEBUGGING_EVALUATION_RESULT_ONLY_MATCH || index == target_index {
                                debug_render_hit_trace(index, &traces);
                            }
                        }
                        if index == target_index {
                            log::info!("GOT A WINNER!");
                            is_correct = true;
                        }
                    }
                }

                move_back = !is_correct;

            } else {
                move_back = true;
            }

            if move_back {
                item.move_back_to_origin();
            }

            if is_correct {
                item.locked.set_neq(true);
                item.play_audio(AudioEffect::Correct);
            } else {
                item.play_audio(AudioEffect::Wrong);
            }
        });

    }
}

pub enum AudioEffect {
    Drag,
    Correct,
    Wrong
}

impl AudioEffect {
    pub fn is_loop(&self) -> bool {
        match self {
            AudioEffect::Drag => true, 
            _ => false
        }
    }

    pub fn as_path(&self) -> AudioPath<'_> {
        let filename = match self {
            AudioEffect::Drag => "drag-loop",
            AudioEffect::Correct => "correct",
            AudioEffect::Wrong => "wrong"
        };

        AudioPath::new_cdn(format!("module/drag-drop/{}.mp3", filename))
    }
}

impl InteractiveItem {

    pub fn play_audio(&self, effect: AudioEffect) {
        *self.audio_effect_handle.borrow_mut() = Some(self.audio_mixer.play(effect.as_path(), effect.is_loop()));
    }

    pub fn stop_audio(&self) {
        *self.audio_effect_handle.borrow_mut() = None; 
    }

    pub fn start_drag(&self, x: i32, y: i32) {
        if !self.locked.get() {
            self.play_audio(AudioEffect::Drag);

            self.drag.set(Some(Rc::new(Drag::new(x, y, 0.0, 0.0, true))));
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

    pub fn try_end_drag(&self, x: i32, y: i32) -> bool {
        self.stop_audio();

        if self.drag.lock_ref().is_some() {
            let drag = self.drag.lock_mut().take().unwrap_ji();
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
