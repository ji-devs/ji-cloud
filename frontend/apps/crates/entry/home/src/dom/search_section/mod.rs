use std::rc::Rc;
use dominator::{html, Dom, clone};
use utils::events;
use futures_signals::signal::SignalExt;

use crate::state::HomePageMode;

use super::super::{
    state::State,
    actions::{fetch_metadata, search}
};


mod categories_select;
mod advanced_search;


pub fn render(state: Rc<State>) -> Dom {

    fetch_metadata(state.clone());

    html!("home-search-section", {
        .property_signal("mode", state.mode.signal_cloned().map(|mode| {
            match mode {
                HomePageMode::Home => "home",
                HomePageMode::Search(_, _) => "results",
            }
        }))
        .child(html!("home-search-section-help", {
            .property("slot", "help")
        }))
        .child(html!("home-search-bar", {
            .property("slot", "search-bar")
            .children(&mut [
                html!("input", {
                    .property("slot", "query")
                    .property("placeholder", "search")
                    .event(clone!(state => move |evt: events::Input| {
                        let v = evt.value();
                        state.search_selected.query.set(v.unwrap_or_default())
                    }))
                }),
                html!("dropdown-select", {
                    .property("slot", "age")
                    .property("label", "_ ages _")
                    .children_signal_vec(state.search_options.age_ranges.signal_cloned().map(clone!(state => move|age_ranges| {
                        age_ranges.iter().map(|age_range| {
                            html!("li-check", {
                                .text(&age_range.display_name)
                                .property_signal("selected", state.search_selected.age_ranges.signal_cloned().map(clone!(age_range => move |age_ranges| {
                                    age_ranges.contains(&age_range.id)
                                })))
                                .event(clone!(state, age_range => move |_: events::Click| {
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
                html!("dropdown-select", {
                    .property("slot", "language")
                    .property("label", "_ languages _")
                    .event(clone!(state => move |_: events::Click| {
                        state.search_selected.language.set(Some("en".to_string()))
                    }))
                    .children(
                        state
                            .search_options
                            .languages
                            .iter()
                            .map(|language| {
                                html!("li-check", {
                                    .text(language)
                                    .property_signal("selected", state.search_selected.language.signal_cloned().map(clone!(language => move |selected_language| {
                                        match selected_language {
                                            Some(selected_language) => selected_language == language,
                                            None => false,
                                        }
                                    })))
                                    .event(clone!(state, language => move |_: events::Click| {
                                        state.search_selected.language.set(Some(language.clone()));
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
