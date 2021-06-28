use dominator::{html, Dom, DomBuilder, clone};
use gloo_timers::future::TimeoutFuture;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlElement;
use std::rc::Rc;
use super::state::*;
use components::module::_groups::cards::{
    lookup::{self, Side},
    play::card::dom::{render_card, render_card_mixin, CardOptions, Size},
    edit::{
        config,
        state::*
    },
};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, ReadOnlyMutable}
};

use shared::domain::jig::module::body::{
    ThemeId,
    ModeExt,
    _groups::cards::{Mode, Step, Card},
};
use rand::prelude::*;

use utils::prelude::*;

pub fn render(state: Rc<Game>) -> Dom {
    html!("card-quiz-main", {
        .property("slot", "main")
        .children_signal_vec(
            state.current.signal_cloned()
                .map(clone!(state => move |current| {
                    let mut children:Vec<Dom> = Vec::new();

                    let theme_id = state.base.theme_id.clone();
                    let mode = state.base.mode.clone();

                    let Current { target, others, side, phase } = current;

                    let mut options = CardOptions::new(&target.card, theme_id, mode, side, Size::QuizTarget);

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
                                //should be some animation
                                .property_signal("flipped", phase.signal().map(clone!(pair_id => move |phase| {
                                    match phase {
                                        CurrentPhase::Correct(id) => if id == pair_id { false } else {true},
                                        CurrentPhase::Wrong(id) => if id == pair_id { false } else {true},
                                        _ => true,
                                    }
                                })))
                                .event(clone!(state, pair_id, phase => move |evt:events::Click| {
                                    if phase.get() == CurrentPhase::Waiting {
                                        spawn_local(clone!(state, pair_id, phase => async move {
                                            if state.check(pair_id) {
                                                phase.set(CurrentPhase::Correct(pair_id));
                                                TimeoutFuture::new(crate::config::SUCCESS_TIME).await;
                                                state.next();
                                            } else {
                                                phase.set(CurrentPhase::Wrong(pair_id));
                                                TimeoutFuture::new(crate::config::WRONG_TIME).await;
                                                phase.set(CurrentPhase::Waiting);
                                            }

                                        }));
                                    } 
                                }))
                        }));
                    }

                    children
                }))
                .to_signal_vec()
        )
    })
}
