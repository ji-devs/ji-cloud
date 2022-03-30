use dominator::{clone, html, with_node, Dom, EventOptions};
use futures_signals::{
    map_ref,
    signal::{Signal, SignalExt},
};
use std::rc::Rc;
use utils::{events, unwrap::UnwrapJiExt};
use web_sys::HtmlInputElement;

use super::super::{
    actions::{fetch_data, search},
    state::State,
};

#[allow(dead_code)] // TODO: remove once advanced search is enabled again
mod advanced_search;
mod categories_select;

const STR_ALL_LANGUAGES: &str = "All languages";
const STR_ALL_AGES: &str = "All ages";
const STR_SEARCH: &str = "Search";
const STR_WHAT_ARE_YOU_LOOKING_FOR: &str = "What are you looking for?";

pub fn render(state: Rc<State>, auto_search: bool) -> Dom {
    fetch_data(state.clone(), auto_search);

    html!("home-search-section", {
        .property_signal("mode", state.mode.signal_cloned().map(|mode| mode.to_string()))
        .property_signal("resultsCount", state.total_jigs_count.signal().map(|x| x as f64))
        // .child(html!("home-search-section-help", {
        //     .property("slot", "help")
        // }))
        .child(html!("form", {
            .property("slot", "search-bar")
            .event_with_options(&EventOptions::preventable(), clone!(state => move |e: events::Submit| {
                e.prevent_default();
                search(Rc::clone(&state));
            }))
            .child(html!("home-search-bar", {
                .children(&mut [
                    html!("input" => HtmlInputElement, {
                        .with_node!(elem => {
                            .property("slot", "query")
                            .property("placeholder", STR_WHAT_ARE_YOU_LOOKING_FOR)
                            // set value on init from query param
                            .property("value", &*state.search_selected.query.lock_ref())
                            .event(clone!(state => move |_: events::Input| {
                                let v = elem.value();
                                state.search_selected.query.set(v)
                            }))
                        })
                    }),
                    html!("home-search-section-select", {
                        .property("slot", "age")
                        .property("multiple", true)
                        .property_signal("value", age_value_signal(state.clone()))
                        .child(html!("input-select-option", {
                            .text(STR_ALL_AGES)
                            .property_signal("selected", state.search_selected.age_ranges.signal_cloned().map(|age_ranges| {
                                age_ranges.is_empty()
                            }))
                            .event(clone!(state => move |_: events::CustomSelectedChange| {
                                state
                                    .search_selected
                                    .age_ranges
                                    .lock_mut()
                                    .clear();
                            }))
                        }))
                        .children_signal_vec(state.search_options.age_ranges.signal_cloned().map(clone!(state => move|age_ranges| {
                            age_ranges.iter().map(|age_range| {
                                let age_id = age_range.id;
                                html!("input-select-option", {
                                    .text(&age_range.display_name)
                                    .property_signal("selected", state.search_selected.age_ranges.signal_cloned().map(clone!(age_range => move |age_ranges| {
                                        age_ranges.contains(&age_range.id)
                                    })))
                                    .event(clone!(state => move |_: events::CustomSelectedChange| {
                                        let mut age_ranges = state.search_selected.age_ranges.lock_mut();
                                        match age_ranges.contains(&age_id) {
                                            true => age_ranges.remove(&age_id),
                                            false => age_ranges.insert(age_id),
                                        };
                                    }))
                                })
                            }).collect()
                        })).to_signal_vec())
                    }),
                    html!("home-search-section-select", {
                        .property("slot", "language")
                        .property_signal("value", language_value_signal(state.clone()))
                        .child(html!("input-select-option", {
                            .text(STR_ALL_LANGUAGES)
                            .property_signal("selected", state.search_selected.language.signal_cloned().map(|lang| lang.is_none()))
                            .event(clone!(state => move |evt: events::CustomSelectedChange| {
                                if evt.selected() {
                                    state.search_selected.language.set(None);
                                }
                            }))
                        }))
                        .children(
                            state
                                .search_options
                                .languages
                                .iter()
                                .map(|lang| {
                                    html!("input-select-option", {
                                        .text(lang.display_name())
                                        .property_signal("selected", state.search_selected.language.signal_cloned().map(clone!(lang => move |selected_language| {
                                            match selected_language {
                                                Some(selected_language) => selected_language == lang.code(),
                                                None => false,
                                            }
                                        })))
                                        .event(clone!(state, lang => move |evt: events::CustomSelectedChange| {
                                            if evt.selected() {
                                                state.search_selected.language.set(Some(lang.code().to_string()));
                                            }
                                        }))
                                    })
                                })
                                .collect::<Vec<Dom>>()
                        )
                    }),
                    html!("button-rect", {
                        .property("slot", "button")
                        .property("size", "small")
                        .property("bold", true)
                        .text(STR_SEARCH)
                        .event(clone!(state => move |_: events::Click| {
                            search(Rc::clone(&state));
                        }))
                    }),
                ])
                // TODO: only disabled for MVP
                // .child(advanced_search::render(state))
            }))
        }))
    })
}

fn age_value_signal(state: Rc<State>) -> impl Signal<Item = String> {
    map_ref! {
        let selected_ages = state.search_selected.age_ranges.signal_cloned(),
        let available_ages = state.search_options.age_ranges.signal_cloned() => {
            let mut output = vec![];
            selected_ages.iter().for_each(|age_id| {
                // only search list if already populated
                if !available_ages.is_empty() {
                    let age = available_ages.iter().find(|age| age.id == *age_id).unwrap_ji();
                    output.push(age.display_name.clone());
                }
            });
            if !output.is_empty() {
                output.join(", ")
            } else {
                STR_ALL_AGES.to_string()
            }
        }
    }
}

fn language_value_signal(state: Rc<State>) -> impl Signal<Item = &'static str> {
    state
        .search_selected
        .language
        .signal_cloned()
        .map(clone!(state => move |selected_language| {
            let lang = state
                .search_options
                .languages
                .iter()
                .find(|lang| match &selected_language {
                    Some(selected_language) => lang.code() == selected_language,
                    None => false,
                });

            match lang {
                Some(lang) => lang.display_name(),
                None => STR_ALL_LANGUAGES
            }
        }))
}
