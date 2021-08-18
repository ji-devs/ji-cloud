use dominator::{Dom, html, clone, with_node};
use futures_signals::signal::{Mutable, SignalExt};
use futures_signals::signal_vec::{SignalVec, SignalVecExt};
use std::ops::Deref;
use std::rc::Rc;
use super::{state::*, actions};
use web_sys::HtmlInputElement;
use utils::{events, routes::*, api_helpers::meta::MetaOptions};
use crate::{
    strings::register::step_3::*,
    register::state::{Step, Step2Data}
};
use wasm_bindgen::prelude::*;

pub struct Step3Page {
}

impl Step3Page {
    pub fn render(step: Mutable<Step>, step_2: Step2Data) -> Dom {
        let state = Rc::new(State::new(step, step_2));

        let meta_options:Mutable<Option<MetaOptions>> = Mutable::new(None);

        html!("page-register-step3", {
            .future(clone!(state, meta_options => async move {
                let meta = MetaOptions::load().await.unwrap_throw();
                state.pre_select(&meta);

                meta_options.set(Some(meta));
            }))
            .children_signal_vec(Self::get_children(meta_options, state))
        })
    }

    fn get_children(meta_options: Mutable<Option<MetaOptions>>, state: Rc<State>) -> impl SignalVec<Item = Dom> {
        meta_options.signal_ref(clone!(state => move |options| {
            match options {
                None => Vec::new(),
                Some(options) => {
                    let mut children:Vec<Dom> = Vec::new();
                    for (id, label) in options.age_ranges.iter() {
                        children.push(
                            html!("input-checkbox", {
                                .property("slot", "ages")
                                .property("label", label)
                                .property("checked", state.age_ranges.borrow().contains(id))
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
                                .property("slot", "affiliations")
                                .property("label", label)
                                .property("checked", state.affiliations.borrow().contains(id))
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
                                .property("slot", "subjects")
                                .property("label", label)
                                .property("checked", state.subjects.borrow().contains(id))
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
                        html!("button-rect", {
                            .property("slot", "submit")
                            .property("color", "red")
                            .property("size", "medium")
                            .text(STR_SUBMIT)
                            .event(clone!(state => move |evt:events::Click| {
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

