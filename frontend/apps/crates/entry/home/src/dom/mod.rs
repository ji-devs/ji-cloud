use std::rc::Rc;
use dominator::{html, Dom, clone};
use futures_signals::signal::SignalExt;

use super::state::{State, HomePageMode};


mod search_section;
mod home_sections;
mod search_results;
mod footer;


pub fn render(state: Rc<State>) -> Dom {
    html!("home-full", {
        .children(&mut [
            search_section::render(state.clone()),
            html!("empty-fragment", {
                .child_signal(state.mode.signal_cloned().map(clone!(state => move |mode| {
                    match mode {
                        HomePageMode::Home => Some(home_sections::render(state.clone())),
                        HomePageMode::Search(query, results) => Some(search_results::render(state.clone(), query, results)),
                    }
                })))
            }),
            footer::render(state.clone()),
        ])
    })
}
