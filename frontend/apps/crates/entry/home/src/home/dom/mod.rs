use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use shared::domain::jig::JigPlayerSettings;
use utils::jig::JigPlayerOptions;
use std::rc::Rc;

use components::{page_footer, page_header::{self, state::PageLinks}, player_popup::{PlayerPopup, PreviewPopupCallbacks}};

use super::state::{HomePageMode, State};

mod home_sections;
mod search_results;
mod search_section;

pub fn render(state: Rc<State>, auto_search: bool) -> Dom {
    html!("home-full", {
        .child(page_header::dom::render(Rc::new(page_header::state::State::new()), None, Some(PageLinks::Home)))
        .children(&mut [
            search_section::render(state.clone(), auto_search),
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
        .child_signal(state.play_jig.signal_cloned().map(clone!(state => move|play_jig| {
            play_jig.map(|jig_id| {
                let close = clone!(state => move || {
                    state.play_jig.set(None);
                });
                PlayerPopup::render(
                    Rc::new(PlayerPopup::new(jig_id, JigPlayerOptions::default(), PreviewPopupCallbacks::new(close))),
                    None
                )
            })
        })))
    })
}
