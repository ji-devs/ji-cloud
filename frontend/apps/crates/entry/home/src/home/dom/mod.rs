use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use utils::events;

use std::rc::Rc;

use components::{
    asset_card::{render_asset_card, AssetCardBottomIndicator, AssetCardConfig},
    page_footer,
    page_header::{PageHeader, PageHeaderConfig, PageLinks},
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
                // Some(page_header::dom::render(Rc::new(page_header::state::PageHeader::new()), None, Some(PageLinks::from(mode)), true))
                Some(PageHeader::new(PageHeaderConfig {
                    active_page: Some(PageLinks::from(mode)),
                    ..Default::default()
                }).render())
            }))
            .child(search_section::render(state.clone(), auto_search))
            .child_signal(state.mode.signal_ref(clone!(state => move |mode| {
                match mode {
                    HomePageMode::Search(_) => None,
                    HomePageMode::Home => {
                        Some(html!("div", {
                            .style("display", "grid")
                            .child(html!("h3", {
                                .style("color", "#fd7076")
                                .style("font-weight", "600")
                                .style("font-size", "24px")
                                .style("margin", "0")
                                .style("padding", "24px")
                                .style("padding-bottom", "0")
                                .text("Trending JIGs")
                            }))
                            .child(html!("div", {
                                .style("overflow-x", "auto")
                                .style("padding", "24px")
                                .style("display", "grid")
                                .style("grid-auto-flow", "column")
                                .style("justify-content", "start")
                                .style("gap", "24px")
                                .style("scrollbar-color", "var(--light-gray-2) transparent")
                                .style("scrollbar-width", "thin")
                                .children_signal_vec(state.trending.signal_cloned().map(clone!(state => move |trending| {
                                    match trending {
                                        None => vec![html!("progress")],
                                        Some(trending) => {
                                            trending.into_iter().map(|jig| {
                                                let jig_id = jig.id;
                                                html!("div", {
                                                    .style("cursor", "pointer")
                                                    .child(render_asset_card(
                                                        &jig.into(),
                                                        AssetCardConfig {
                                                            bottom_indicator: AssetCardBottomIndicator::Author,
                                                            dense: true,
                                                            ..Default::default()
                                                        }
                                                    ))
                                                    .event(clone!(state => move |_: events::Click| {
                                                        state.play_asset.set(Some(jig_id.into()));
                                                    }))
                                                })
                                            }).collect()
                                        },
                                    }
                                })).to_signal_vec())
                            }))
                        }))
                    }
                }
            })))
            .child(html!("empty-fragment", {
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
            }))
            .child(page_footer::dom::render(None))
            .child_signal(state.play_asset.signal_cloned().map(clone!(state => move|play_asset| {
                play_asset.map(|asset_id| {
                    let close = clone!(state => move || {
                        state.play_asset.set(None);
                    });
                    PlayerPopup::new_default_player_options_with_jig_quota(
                        asset_id,
                        PreviewPopupCallbacks::new(close)
                    ).render(None)
                })
            })))
        })
    }
}
