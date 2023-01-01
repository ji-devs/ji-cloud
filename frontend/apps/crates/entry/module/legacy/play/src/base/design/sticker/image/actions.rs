use super::state::Controller;
use std::sync::atomic::Ordering;

use crate::base::actions::{StageClick, StageClickContinuation};
use shared::domain::module::body::legacy::design::HideToggle;
use utils::math::bounds::BoundsF64;

impl Controller {
    pub fn handle_click(&self, stage_click: StageClick) -> StageClickContinuation {
        let mut continuation = StageClickContinuation::KeepGoing;
        let is_target = {
            match self.elem.borrow().as_ref() {
                None => false,
                Some(elem) => {
                    let bounds: BoundsF64 = elem.into();
                    bounds.contains_point(stage_click.mouse_x, stage_click.mouse_y)
                }
            }
        };

        if !is_target || !self.interactive {
            return continuation;
        }

        let has_toggled_once = self.has_toggled_once.load(Ordering::SeqCst);

        let mut toggle_triggered = false;

        if let Some(hide_toggle) = self.hide_toggle {
            if !has_toggled_once || hide_toggle == HideToggle::Always {
                let val = self.hidden.get();
                self.hidden.set(!val);
                toggle_triggered = true;
                continuation = StageClickContinuation::Stop;
            }
        }

        self.has_toggled_once.store(true, Ordering::SeqCst);

        // in theory we would only play sound for visible images
        // i.e. the first tuple element should be `self.hidden.get()`
        // but it turns out audio is also supposed to play when the image gets hidden
        // so we use toggle_triggered instead
        match (toggle_triggered, self.audio_filename.as_ref()) {
            (true, Some(audio_filename)) => {
                //win the race condition with hotspots
                self.base
                    .audio_manager
                    .play_clip_next_tick(self.base.design_media_url(&audio_filename));
            }
            _ => {}
        }

        continuation
    }
}
