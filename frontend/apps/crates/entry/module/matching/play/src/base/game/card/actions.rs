use super::{super::state::*, state::*};
use components::audio::mixer::{play_random_negative, play_random_positive, AUDIO_MIXER};
use std::rc::Rc;
use utils::{
    math::BoundsF64,
    prelude::{IframeAction, IframeMessageExt, ModuleToJigPlayerMessage},
    unwrap::UnwrapJiExt,
};
use web_sys::HtmlElement;

impl CardDrag {
    pub fn on_release(&self) {
        if let Some(current) = self.game.get_current() {
            let top = current.top.iter().find(|choice| choice.is_drag_over());
            // Impossible to be none, you can only drag a bottom.
            let bottom = current
                .bottom
                .iter()
                .find(|choice| choice.pair_id == self.pair_id)
                .unwrap_ji();

            // place card
            match top {
                Some(top) if top.pair_id == self.pair_id => {
                    top.phase.set(TopPhase::Landed);
                }
                _ => {
                    if let Some(target) = current
                        .bottom
                        .iter()
                        .find(|choice| choice.pair_id == self.pair_id)
                    {
                        target.phase.set(BottomPhase::Show);
                    }
                }
            };

            let mut report = self.game.base.play_report.lock_mut();
            let card_report = &mut report.rounds.last_mut().unwrap().get_mut(&self.pair_id);
            let card_report = card_report.as_mut().unwrap();

            if let Some(top) = top {
                if top.pair_id == self.pair_id {
                    play_random_positive();

                    let points = calculate_point_count(*bottom.tried_count.borrow());
                    let _ = IframeAction::new(ModuleToJigPlayerMessage::AddPoints(points))
                        .try_post_message_to_player();

                    // card_report.succeeded = true;
                } else {
                    play_random_negative();

                    bottom
                        .tried_count
                        .replace_with(|tried_count| *tried_count + 1);

                    card_report.failed_tries += 1;
                }
            } else {
                // Only treat as failed if they've dropped the card over a target. If
                // they drop the card over nothing, it could be for something like releasing
                // the card to select a new card.
            }

            // Game::next needs access to report.
            drop(report);
            if current.top.iter().all(|choice| choice.is_landed()) {
                Game::next(self.game.clone());
            }

            current.drag.set(None);
        }
    }

    // picks the first hit in order of the Vec, not graphical majority
    pub fn evaluate_drag_over(&self) {
        match (self.game.get_current(), self.get_bounds()) {
            (Some(current), Some(src_bounds)) => {
                let mut found_drag_over = false;

                for choice in current.top.iter() {
                    if found_drag_over {
                        choice.set_drag_over(false);
                    } else {
                        let is_drag_over = {
                            if let Some(elem) = choice.elem.borrow().as_ref() {
                                let target_bounds: BoundsF64 =
                                    elem.get_bounding_client_rect().into();
                                if src_bounds.intersects(target_bounds) {
                                    found_drag_over = true;
                                    true
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        };

                        choice.set_drag_over(is_drag_over);
                    }
                }
            }
            _ => {}
        }
    }
}

pub fn start_drag(state: Rc<CardBottom>, elem: HtmlElement, x: i32, y: i32) {
    state.phase.set(BottomPhase::Remove);
    if let Some(current) = state.game.get_current() {
        current
            .drag
            .set(Some(Rc::new(CardDrag::new((*state).clone(), elem, x, y))));

        if let Some(audio) = &state.card.audio {
            AUDIO_MIXER.with(|mixer| mixer.play_oneshot(audio.into()));
        }
    }
}

fn calculate_point_count(tried_count: u32) -> u32 {
    // start with 2 point, reduce one point for every try. min points: 0.
    let base = 2_u32;
    base.saturating_sub(tried_count)
}
