use dominator::{clone, html, with_node, Dom};
use futures_signals::{
    map_ref,
    signal::{Signal, SignalExt},
};
use std::rc::Rc;
use utils::{events, unwrap::UnwrapJiExt};
use web_sys::HtmlInputElement;

use super::super::state::HomePageMode;

use super::super::{
    actions::{fetch_data, search},
    state::State,
};

mod advanced_search;
mod categories_select;

const STR_ALL_LANGUAGES: &'static str = "All languages";
const STR_ALL_AGES: &'static str = "All ages";

pub fn render(state: Rc<State>) -> Dom {
    fetch_data(state.clone());

    html!("home-search-section", {
        .property_signal("mode", state.mode.signal_cloned().map(|mode| {
            match mode {
                HomePageMode::Home => "home",
                HomePageMode::Search(_, _) => "results",
            }
        }))
        .property_signal("resultsCount", state.total_jigs_count.signal().map(|x| x as f64))
        .child(html!("home-search-section-help", {
            .property("slot", "help")
        }))
        .child(html!("home-search-bar", {
            .property("slot", "search-bar")
            .children(&mut [
                html!("input" => HtmlInputElement, {
                    .with_node!(elem => {
                        .property("slot", "query")
                        .property("placeholder", "search")
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
                    .children_signal_vec(state.search_options.age_ranges.signal_cloned().map(clone!(state => move|age_ranges| {
                        age_ranges.iter().map(|age_range| {
                            html!("input-select-option", {
                                .text(&age_range.display_name)
                                .property_signal("selected", state.search_selected.age_ranges.signal_cloned().map(clone!(age_range => move |age_ranges| {
                                    age_ranges.contains(&age_range.id)
                                })))
                                .event(clone!(state, age_range => move |_: events::CustomSelectedChange| {
                                    let mut age_ranges = state.search_selected.age_ranges.lock_mut();
                                    match age_ranges.contains(&age_range.id) {
                                        true => age_ranges.remove(&age_range.id),
                                        false => age_ranges.insert(age_range.id.clone()),
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
                    .property("bold", true)
                    .text("Search")
                    .event(clone!(state => move |_: events::Click| {
                        search(Rc::clone(&state));
                    }))
                }),
            ])
            .child(advanced_search::render(state.clone()))
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
                if available_ages.len() > 0 {
                    let age = available_ages.iter().find(|age| age.id == *age_id).unwrap_ji();
                    output.push(age.display_name.clone());
                }
            });
            if output.len() > 0 {
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
