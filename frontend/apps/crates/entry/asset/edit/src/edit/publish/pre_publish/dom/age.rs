use dominator::{clone, html, Dom};
use futures_signals::{
    map_ref,
    signal::{Signal, SignalExt},
};
use shared::domain::meta::AgeRange;
use std::rc::Rc;
use utils::{events, unwrap::UnwrapJiExt};

use super::super::state::PrePublish;

const STR_AGE_LABEL: &str = "Age range";
const STR_AGE_PLACEHOLDER: &str = "Select one or more";
const STR_ALL_AGES: &str = "All ages";

impl PrePublish {
    pub fn render_ages(self: Rc<Self>) -> Dom {
        let state = Rc::clone(&self);
        html!("input-select", {
            .prop("slot", "age")
            .prop("label", STR_AGE_LABEL)
            .prop("placeholder", STR_AGE_PLACEHOLDER)
            .prop("multiple", true)
            .prop_signal("value", age_value_signal(state.clone()))
            // .prop_signal("error", {
            //     (map_ref! {
            //         let submission_tried = state.submission_tried.signal(),
            //         let value = state.jig.age_ranges.signal_cloned()
            //             => (*submission_tried, value.clone())
            //     })
            //         .map(|(submission_tried, value)| {
            //             submission_tried && value.is_empty()
            //         })
            // })
            .child(html!("input-select-option", {
                .text(STR_ALL_AGES)
                .prop_signal("selected", state.asset.age_ranges().signal_cloned().map(|age_ranges| {
                    age_ranges.is_empty()
                }))
                .event(clone!(state => move |_: events::CustomSelectedChange| {
                    state
                        .asset
                        .age_ranges()
                        .lock_mut()
                        .clear();
                }))
            }))
            .children_signal_vec(state.ages.signal_cloned().map(clone!(state => move |ages| {
                ages.iter().map(|age| {
                    render_age(age, state.clone())
                }).collect()
            })).to_signal_vec())
        })
    }
}

fn render_age(age: &AgeRange, state: Rc<PrePublish>) -> Dom {
    let age_id = age.id;
    html!("input-select-option", {
        .text(&age.display_name)
        .prop_signal("selected", state.asset.age_ranges().signal_cloned().map(clone!(age_id => move |ages| {
            ages.contains(&age_id)
        })))
        .event(clone!(state => move |_: events::CustomSelectedChange| {
            let mut ages = state.asset.age_ranges().lock_mut();
            if ages.contains(&age_id) {
                ages.remove(&age_id);
            } else {
                ages.insert(age_id);
            }
        }))
    })
}

fn age_value_signal(state: Rc<PrePublish>) -> impl Signal<Item = String> {
    map_ref! {
        let selected_ages = state.asset.age_ranges().signal_cloned(),
        let available_ages = state.ages.signal_cloned() => {
            let mut output = vec![];
            selected_ages.iter().for_each(|age_id| {
                let age = available_ages.iter().find(|age| age.id == *age_id).unwrap_ji();
                output.push(age.display_name.clone());
            });
            output.join(", ")
        }

    }
}
