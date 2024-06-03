use dominator::{clone, html, Dom};
use futures_signals::{signal::SignalExt, signal_vec::SignalVecExt};
use std::rc::Rc;
use utils::init::user::get_user_id;
use utils::{events, prelude::get_user_cloned};

use super::super::super::dom::render_flippable_asset_card;
use super::state::SearchResultsSection;

const STR_LOAD_MORE: &str = "See more";

impl SearchResultsSection {
    pub fn render(self: &Rc<Self>) -> Dom {
        let state = self;
        let user_id = get_user_id();

        // Only set this once, but I don't want to add once_cell crate when it's not really needed.
        state.user.set(get_user_cloned());

        html!("home-search-results-section", {
            .prop("slot", "sections")
            .prop("kind", state.asset_type.as_str())
            .prop_signal("resultsCount", state.total.signal())
            .children_signal_vec(state.list.signal_vec_cloned().map(clone!(state => move |jig| {
                let jig_id = jig.id();
                render_flippable_asset_card(jig, user_id, Box::new(clone!(state => move || {
                    state.home_state.play_asset.set(Some(jig_id.into()));
                })))
            })))
            .child_signal(state.all_loaded_signal().map(clone!(state => move |all_loaded| {
                match all_loaded {
                    true => None,
                    false => {
                        Some(html!("button-rect", {
                            .prop("slot", "load-more")
                            .prop("color", "blue")
                            .prop("type", "filled")
                            .prop_signal("disabled", state.loader.is_loading())
                            .text(STR_LOAD_MORE)
                            .event(clone!(state => move |_: events::Click| {
                                state.loader.load(clone!(state => async move {
                                    state.load_items().await;
                                }));
                            }))
                        }))
                    },
                }
            })))
        })
    }
}
