use dominator::{clone, html, Dom};

use super::state::*;
use components::module::_groups::cards::play::card::dom::{
    render_card, render_card_mixin, CardOptions, Effect, Size,
};
use futures_signals::signal::SignalExt;
use std::rc::Rc;

use utils::prelude::*;

impl Game {
    pub fn render(state: Rc<Self>) -> Dom {
        html!("card-quiz-main", {
            .property("slot", "main")
            .children_signal_vec(
                state.current.signal_cloned()
                    .map(clone!(state => move |current| {
                        let mut children:Vec<Dom> = Vec::new();

                        if let Some(current) = current {
                            let theme_id = state.base.theme_id;
                            let mode = state.base.mode;

                            let Current { target, others, side, phase, .. } = &*current;

                            let mut options = CardOptions::new(&target.card, theme_id, mode, *side, Size::QuizTarget);

                            options.flipped = true;
                            options.slot = Some("target");

                            children.push(render_card(options));

                            for other in others.iter() {

                                let mut options = CardOptions::new(&other.card, theme_id, mode, side.negate(), Size::QuizOption);

                                options.flipped = true;
                                options.slot = Some("options");

                                let pair_id = other.pair_id;

                                children.push(render_card_mixin(options, |dom| {
                                    dom
                                        .property_signal(
                                            "effect",
                                            phase.signal().map(move |phase| {
                                                match phase {
                                                    CurrentPhase::Correct(id) if id == pair_id => Effect::Positive,
                                                    _ => Effect::None,
                                                }
                                            })
                                        )
                                        .property_signal("flipped", phase.signal().map(clone!(state, pair_id => move |phase| {
                                            if is_incorrect_choice(&state, &pair_id) {
                                                false
                                            } else {
                                                match phase {
                                                    CurrentPhase::Correct(id) => id == pair_id,
                                                    _ => true,
                                                }
                                            }

                                        })))
                                        .event(clone!(state, pair_id, phase => move |_evt:events::Click| {
                                            Self::evaluate(state.clone(), pair_id, phase.clone());
                                        }))
                                }));
                            }
                        }

                        children
                    }))
                    .to_signal_vec()
            )
        })
    }
}

fn is_incorrect_choice(state: &Rc<Game>, pair_id: &usize) -> bool {
    state
        .current
        .lock_ref()
        .as_ref()
        .unwrap_ji()
        .incorrect_choices
        .borrow()
        .iter()
        .find(|id| *id == pair_id)
        .is_some()
}
