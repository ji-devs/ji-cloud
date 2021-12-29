use dominator::{clone, html, Dom};
use futures_signals::{
    map_ref,
    signal::{Signal, SignalExt},
};
use std::rc::Rc;
use utils::{events, unwrap::UnwrapJiExt};

use super::categories_select;

use super::super::super::{actions::search, state::State};

const STR_SEARCH: &str = "Advanced Search";
const STR_GOAL_LABEL: &str = "Teaching Goal";
const STR_GOAL_PLACEHOLDER: &str = "Select from the list";
const STR_AFFILIATION_LABEL: &str = "Affiliation";
const STR_AFFILIATION_PLACEHOLDER: &str = "Select one or more from the list";

pub fn render(state: Rc<State>) -> Dom {
    html!("home-search-section-advanced", {
        .property("slot", "advanced")
        .children(&mut [
            html!("button-rect", {
                .attribute("style", "height: 48px")
                .property("slot", "opener")
                .property("kind", "text")
                .property("color", "white")
                .property("bold", true)
                .text(STR_SEARCH)
            }),

            html!("input-select", {
                .property("slot", "affiliation")
                .property("label", STR_AFFILIATION_LABEL)
                .property("placeholder", STR_AFFILIATION_PLACEHOLDER)
                .property("multiple", true)
                .visible_signal(state.is_logged_in.signal().map(|is_logged_in| {
                    !is_logged_in
                }))
                .property_signal("value", affiliation_value_signal(state.clone()))
                .children_signal_vec(state.search_options.affiliations.signal_cloned().map(clone!(state => move|affiliations| {
                    affiliations.iter().map(|affiliation| {
                        html!("input-select-option", {
                            .text(&affiliation.display_name)
                            .property_signal("selected", state.search_selected.affiliations.signal_cloned().map(clone!(affiliation => move |affiliations| {
                                affiliations.contains(&affiliation.id)
                            })))
                            .event(clone!(state, affiliation => move |_: events::CustomSelectedChange| {
                                let mut affiliations = state.search_selected.affiliations.lock_mut();
                                match affiliations.contains(&affiliation.id) {
                                    true => affiliations.remove(&affiliation.id),
                                    false => affiliations.insert(affiliation.id),
                                };
                            }))
                        })
                    }).collect()
                })).to_signal_vec())
            }),

            html!("input-select", {
                .property("slot", "goal")
                .property("label", STR_GOAL_LABEL)
                .property("placeholder", STR_GOAL_PLACEHOLDER)
                .property("multiple", true)
                .property_signal("value", goal_value_signal(state.clone()))
                .children_signal_vec(state.search_options.goals.signal_cloned().map(clone!(state => move|goals| {
                    goals.iter().map(|goal| {
                        html!("input-select-option", {
                            .text(&goal.display_name)
                            .property_signal("selected", state.search_selected.goals.signal_cloned().map(clone!(goal => move |goals| {
                                goals.contains(&goal.id)
                            })))
                            .event(clone!(state, goal => move |_: events::CustomSelectedChange| {
                                let mut goals = state.search_selected.goals.lock_mut();
                                match goals.contains(&goal.id) {
                                    true => goals.remove(&goal.id),
                                    false => goals.insert(goal.id),
                                };
                            }))
                        })
                    }).collect()
                })).to_signal_vec())
            }),

            categories_select::render(state.clone()),

            html!("button-rect", {
                .property("slot", "search-button")
                .property("color", "blue")
                .text(STR_SEARCH)
                .event(clone!(state => move |_: events::Click| {
                    search(Rc::clone(&state));
                }))
            })
        ])
    })
}

fn goal_value_signal(state: Rc<State>) -> impl Signal<Item = String> {
    map_ref! {
        let selected_goals = state.search_selected.goals.signal_cloned(),
        let available_goals = state.search_options.goals.signal_cloned() => {
            let mut output = vec![];
            selected_goals.iter().for_each(|goal_id| {
                // only search list if already populated
                if !available_goals.is_empty() {
                    let goal = available_goals.iter().find(|goal| goal.id == *goal_id).unwrap_ji();
                    output.push(goal.display_name.clone());
                }
            });
            output.join(", ")
        }

    }
}

fn affiliation_value_signal(state: Rc<State>) -> impl Signal<Item = String> {
    map_ref! {
        let selected_affiliations = state.search_selected.affiliations.signal_cloned(),
        let available_affiliations = state.search_options.affiliations.signal_cloned() => {
            let mut output = vec![];
            selected_affiliations.iter().for_each(|affiliation_id| {
                // only search list if already populated
                if !available_affiliations.is_empty() {
                    let affiliation = available_affiliations.iter().find(|affiliation| affiliation.id == *affiliation_id).unwrap_ji();
                    output.push(affiliation.display_name.clone());
                }
            });
            output.join(", ")
        }

    }
}
