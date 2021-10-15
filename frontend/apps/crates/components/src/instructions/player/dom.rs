use dominator::{clone, html, Dom};

use std::rc::Rc;
use utils::prelude::*;

use super::state::*;

impl InstructionsPlayer {
    pub fn render(state: Rc<Self>) -> Dom {
        state.reset_ended();
        Self::play_audio(state.clone());
        state.evaluate_all_ended();

        html!("empty-fragment", {
            .apply_if(state.data.text.is_some(), |dom| {
                let text = state.data.text.as_ref().unwrap_ji();

                state.fade.render(dom.child(
                    html!("instructions-banner", {
                        .text(text)
                    })
                ))

            })
            .after_removed(clone!(state => move |_elem| {
                *state.audio.borrow_mut() = None;
            }))
        })
    }
}
