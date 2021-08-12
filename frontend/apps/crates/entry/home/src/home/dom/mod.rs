use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use std::rc::Rc;

use components::{page_header, page_footer};

use super::state::{HomePageMode, State};

mod home_sections;
mod search_results;
mod search_section;

pub fn render(state: Rc<State>) -> Dom {
    html!("home-full", {
        .child(page_header::dom::render(Rc::new(page_header::state::State::new()), None))
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
            page_footer::dom::render(None),
        ])
    })
}
