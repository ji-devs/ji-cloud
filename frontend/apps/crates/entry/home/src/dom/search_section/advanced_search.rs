use std::rc::Rc;
use dominator::{html, Dom, clone};
use futures_signals::signal::SignalExt;
use utils::events;

use super::categories_select;

use crate::state::State;

pub fn render(state: Rc<State>) -> Vec<Dom> {
    vec![

        html!("dropdown-select", {
            .property("label", "_ affiliation _")
            .children_signal_vec(state.search_options.affiliations.signal_cloned().map(clone!(state => move|affiliations| {
                affiliations.iter().map(|affiliation| {
                    html!("li-check", {
                        .text(&affiliation.display_name)
                        .property_signal("selected", state.search_selected.affiliations.signal_cloned().map(clone!(affiliation => move |affiliations| {
                            affiliations.contains(&affiliation.id)
                        })))
                        .event(clone!(state, affiliation => move |_: events::Click| {
                            let mut affiliations = state.search_selected.affiliations.lock_mut();
                            match affiliations.contains(&affiliation.id) {
                                true => affiliations.remove(&affiliation.id),
                                false => affiliations.insert(affiliation.id.clone()),
                            };
                        }))
                    })
                }).collect()
            })).to_signal_vec())
        }),

        html!("dropdown-select", {
            .property("label", "_ goal _")
            .children_signal_vec(state.search_options.goals.signal_cloned().map(clone!(state => move|goals| {
                goals.iter().map(|goal| {
                    html!("li-check", {
                        .text(&goal.display_name)
                        .property_signal("selected", state.search_selected.goals.signal_cloned().map(clone!(goal => move |goals| {
                            goals.contains(&goal.id)
                        })))
                        .event(clone!(state, goal => move |_: events::Click| {
                            let mut goals = state.search_selected.goals.lock_mut();
                            match goals.contains(&goal.id) {
                                true => goals.remove(&goal.id),
                                false => goals.insert(goal.id.clone()),
                            };
                        }))
                    })
                }).collect()
            })).to_signal_vec())
        }),

        categories_select::render(state.clone()),

    ]
}
