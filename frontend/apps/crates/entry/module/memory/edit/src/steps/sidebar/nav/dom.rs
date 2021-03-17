use dominator::{html, Dom, clone};
use crate::data::*;
use std::rc::Rc;
use utils::events;
use wasm_bindgen::prelude::*;
use futures_signals::signal::SignalExt;

pub struct StepsNavDom {}
impl StepsNavDom {
    pub fn render(state:Rc<State>) -> Dom {
        html!("steps-nav", {
            .property("slot", "nav")
            .children(
                [Step::One, Step::Two, Step::Three, Step::Four]
                    .into_iter()
                    .map(|step| {
                        html!("step-nav", {
                            .property("number", JsValue::from_f64(step.number() as f64))
                            .property("label", step.label())
                            .property_signal("active", state.step.signal().map(move |curr| {
                                if curr == *step {
                                    true
                                } else {
                                    false
                                }
                            }))
                            .property_signal("completed", state.steps_completed.signal_ref(move |steps_completed| {
                                steps_completed.contains(step)
                            }))
                            .event(clone!(state => move |evt:events::Click| {
                                state.change_step(*step);
                            }))
                        })
                    })
                    .collect::<Vec<Dom>>()
            )
        })
    }
}
