use components::asset_card::{render_asset_card, AssetCardConfig};
use dominator::{clone, html, Dom, EventOptions};
use futures_signals::{signal::SignalExt, signal_vec::SignalVecExt};
use shared::domain::asset::{Asset, AssetId};
use std::rc::Rc;
use utils::{asset::ResourceContentExt, events};

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
                html!("a", {
                    .prop("slot", "results")
                    .style("cursor", "pointer")
                    .apply(|mut dom| {
                        if let Asset::Resource(resource) = asset.as_ref() {
                            if let Some(resource) = resource.resource_data.additional_resources.get(0) {
                                dom = dom
                                    .style("text-decoration", "none")
                                    .prop("target", "_blank")
                                    .prop("href", resource.resource_content.get_link());
                            }
                        }
                        dom
                    })
                    .child(render_asset_card(&asset, AssetCardConfig {
                        dense: false,
                        menu: Some(Rc::new(clone!(state => move || {
                            html!("menu-kebab", {
                                .prop("slot", "menu")
                                .children(&mut [
                                    html!("menu-line", {
                                        .prop("icon", "unlike")
                                        .event(clone!(state => move |_: events::Click| {
                                            state.unlike(asset_id);
                                        }))
                                    })
                                ])
                            })
                        }))),
                        ..Default::default()
                    }))
                    .event_with_options(&EventOptions::bubbles(), clone!(state => move |_: events::Click| {
                        match asset_id {
                            AssetId::JigId(_) | AssetId::PlaylistId(_) | AssetId::CourseId(_) => {
                                state.play_asset.set(Some(asset_id));
                            },
                            AssetId::ResourceId(_) => {},
                        }
                    }))
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
