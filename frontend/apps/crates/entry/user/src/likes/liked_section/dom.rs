use components::asset_card::{render_asset_card, AssetCardConfig};
use dominator::{clone, html, Dom};
use futures_signals::{signal::SignalExt, signal_vec::SignalVecExt};
use std::rc::Rc;
use utils::events;

use super::state::LikedSection;

const STR_LOAD_MORE: &str = "See more";

impl LikedSection {
    pub fn render(self: &Rc<Self>) -> Dom {
        let state = self;

        html!("home-search-results-section", {
            .prop("slot", "sections")
            .prop("kind", state.asset_type.as_str())
            .prop_signal("resultsCount", state.total.signal())
            .children_signal_vec(state.list.signal_vec_cloned().map(clone!(state => move |asset| {
                let asset_id = asset.id();
                render_asset_card(&asset, AssetCardConfig {
                    slot: Some("results"),
                    dense: true,
                    menu: Some(Rc::new(clone!(state => move || {
                        html!("menu-kebab", {
                            .prop("slot", "menu")
                            .children(&mut [
                                html!("menu-line", {
                                    .prop("icon", "jig-play")
                                    .event(clone!(state => move |_: events::Click| {
                                        state.unlike(asset_id);
                                    }))
                                })
                            ])
                        })
                    }))),
                    ..Default::default()
                })
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
