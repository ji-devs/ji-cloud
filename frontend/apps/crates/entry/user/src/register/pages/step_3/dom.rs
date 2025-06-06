use dominator::{clone, html, Dom};
use futures_signals::signal::{Mutable, SignalExt};
use futures_signals::signal_vec::SignalVec;
use utils::unwrap::UnwrapJiExt;

use super::{actions, state::*};
use std::rc::Rc;

use crate::{register::state::Step2Data, strings::register::step_3::*};
use utils::{api_helpers::meta::MetaOptions, events};

pub struct Step3Page {}

impl Step3Page {
    pub fn render(step_2: Step2Data) -> Dom {
        let state = Rc::new(State::new(step_2));

        let meta_options: Mutable<Option<MetaOptions>> = Mutable::new(None);

        html!("page-register-step3", {
            .future(clone!(meta_options => async move {
                let meta = MetaOptions::load().await.unwrap_ji();
                meta_options.set(Some(meta));
            }))
            .children_signal_vec(Self::get_children(meta_options, state))
        })
    }

    fn get_children(
        meta_options: Mutable<Option<MetaOptions>>,
        state: Rc<State>,
    ) -> impl SignalVec<Item = Dom> {
        meta_options
            .signal_ref(clone!(state => move |options| {
                match options {
                    None => Vec::new(),
                    Some(options) => {
                        let mut children:Vec<Dom> = Vec::new();
                        for (id, label) in options.age_ranges.iter() {
                            children.push(
                                html!("input-checkbox", {
                                    .prop("slot", "ages")
                                    .prop("label", label)
                                    .prop("checked", state.age_ranges.borrow().contains(id))
                                    .event(clone!(state, id => move |evt:events::CustomToggle| {
                                        if evt.value() {
                                            state.age_ranges.borrow_mut().insert(id.clone());
                                        } else {
                                            state.age_ranges.borrow_mut().remove(&id);
                                        }
                                    }))
                                })
                            )
                        }
                        for (id, label) in options.affiliations.iter() {
                            children.push(
                                html!("input-checkbox", {
                                    .prop("slot", "affiliations")
                                    .prop("label", label)
                                    .prop("checked", state.affiliations.borrow().contains(id))
                                    .event(clone!(state, id => move |evt:events::CustomToggle| {
                                        if evt.value() {
                                            state.affiliations.borrow_mut().insert(id.clone());
                                        } else {
                                            state.affiliations.borrow_mut().remove(&id);
                                        }
                                    }))
                                })
                            )
                        }
                        for (id, label) in options.subjects.iter() {
                            children.push(
                                html!("input-checkbox", {
                                    .prop("slot", "subjects")
                                    .prop("label", label)
                                    .prop("checked", state.subjects.borrow().contains(id))
                                    .event(clone!(state, id => move |evt:events::CustomToggle| {
                                        if evt.value() {
                                            state.subjects.borrow_mut().insert(id.clone());
                                        } else {
                                            state.subjects.borrow_mut().remove(&id);
                                        }
                                    }))
                                })
                            )
                        }

                        children.push(
                            html!("button-rect-icon", {
                                .prop("slot", "submit")
                                .prop("color", "red")
                                .prop("size", "regular")
                                .prop("iconAfter", "arrow")
                                .text(STR_SUBMIT)
                                .event(clone!(state => move |_evt:events::Click| {
                                    actions::submit(state.clone());
                                }))
                            })
                        );
                        children
                    }
                }
            }))
            .to_signal_vec()
    }
}
