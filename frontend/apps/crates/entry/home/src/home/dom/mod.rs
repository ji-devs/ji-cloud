use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;

use std::rc::Rc;
use utils::events;

use components::{
    overlay::handle::OverlayHandle,
    page_footer,
    page_header::{self, state::PageLinks},
    player_popup::{PlayerPopup, PreviewPopupCallbacks},
};

use super::state::{Home, HomePageMode};

mod home_sections;
mod iframe;
mod search_section;
use iframe::Iframe;

impl Home {
    pub fn render(self: Rc<Self>, auto_search: bool) -> Dom {
        let state = self;
        html!("home-full", {
            .child_signal(state.mode.signal_ref(|mode| {
                Some(page_header::dom::render(Rc::new(page_header::state::State::new()), None, Some(PageLinks::from(mode)), true))
            }))
            .children(&mut [
                search_section::render(state.clone(), auto_search),
                html!("empty-fragment", {
                    .child_signal(state.mode.signal_cloned().map(move |mode| {
                        match mode {
                            HomePageMode::Home => {
                                // Some(home_sections::render(state.clone()))
                                Some(Iframe::new().render())
                            },
                            HomePageMode::Search(search_results) => {
                                Some(search_results.render())
                            },
                        }
                    }))
                }),
                page_footer::dom::render(None),
            ])
            .child_signal(state.play_asset.signal_cloned().map(clone!(state => move|play_asset| {
                play_asset.map(|asset_id| {
                    let close = clone!(state => move || {
                        state.play_asset.set(None);
                    });
                    PlayerPopup::new_default_player_options(
                        asset_id,
                        PreviewPopupCallbacks::new(close)
                    ).render(None)
                })
            })))
            .child_signal(state.play_login_popup_shown.signal().map(move|play_login_popup_shown| {
                match play_login_popup_shown {
                    false => None,
                    true => {
                        Some(html!("empty-fragment", {
                            .style("display", "none")
                            .apply(OverlayHandle::lifecycle(clone!(state => move || {
                                html!("home-login-before-play", {
                                    .child(html!("fa-button", {
                                        .prop("slot", "close")
                                        .prop("icon", "fa-solid fa-xmark")
                                        .event(clone!(state => move |_: events::Click| {
                                            state.play_login_popup_shown.set(false);
                                        }))
                                    }))
                                })
                            })))
                        }))
                    },
                }
            }))
        })
    }
}
