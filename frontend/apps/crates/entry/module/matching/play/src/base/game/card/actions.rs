use super::{super::state::*, state::*};
use std::rc::Rc;
use utils::math::BoundsF64;
use web_sys::HtmlElement;

impl CardDrag {
    pub fn on_release(&self) {
        if let Some(current) = self.game.get_current() {
            let choice = current.top.iter().find(|choice| choice.is_drag_over());

            let mut found_match = false;

            if let Some(choice) = choice {
                if choice.pair_id == self.pair_id {
                    found_match = true;
                    choice.phase.set(TopPhase::Landed);
                } else {
                    //specific wrong answer
                }
            } else {
                //empty area
            }

            if !found_match {
                if let Some(target) = current
                    .bottom
                    .iter()
                    .find(|choice| choice.pair_id == self.pair_id)
                {
                    target.phase.set(BottomPhase::Show);
                }
            } else if current.top.iter().all(|choice| choice.is_landed()) {
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
    }
}
