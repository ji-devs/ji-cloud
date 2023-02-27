use std::rc::Rc;

use components::{
    asset_card::{render_asset_card, AssetCardBottomIndicator, AssetCardConfig},
    player_popup::{PlayerPopup, PreviewPopupCallbacks},
};
use dominator::{clone, html, Dom, EventOptions};
use futures_signals::{
    map_ref,
    signal::{Signal, SignalExt},
    signal_vec::SignalVecExt,
};
use shared::domain::{
    asset::{Asset, AssetId},
    jig::JigResponse,
};
use utils::{
    asset::{AssetPlayerOptions, JigPlayerOptions},
    events,
    unwrap::UnwrapJiExt,
};

use super::state::CourseSelection;

const STR_LOAD_MORE: &str = "See more";

impl CourseSelection {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;

        state.search();

        let search_callback: Rc<dyn Fn()> = Rc::new(clone!(state => move || {
            state.next_page.set(0);
            state.search_results.lock_mut().clear();
            state.search();
        }));

        html!("asset-edit-course-selection", {
            .prop("slot", "main")
            .child(state.search_bar.render_bar(Rc::clone(&search_callback)))
            .child(state.search_bar.render_rated_toggle(Rc::clone(&search_callback), Some("rated")))
            .child(html!("home-search-results", {
                .prop("slot", "results")
                .prop_signal("jigCount", state.total_jig_count.signal())
                .prop_signal("query", state.active_query.signal_cloned())
                .child(html!("home-search-results-section", {
                    .prop("slot", "sections")
                    .prop("kind", "jig")
                    .prop("dense", true)
                    .prop_signal("resultsCount", state.total_jig_count.signal())
                    .children_signal_vec(state.search_results.signal_vec_cloned().map(clone!(state => move |jig| {
                        state.render_asset(&jig)
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
                                        state.search();
                                    }))
                                }))
                            },
                        }
                    })))
                }))
            }))
            .child_signal(state.drag.signal_ref(clone!(state => move|drag| {
                drag.as_ref().map(|drag| {
                    let asset = &drag.data;

                    html!("div", {
                        .prop("slot", "dragging")
                        .style_signal("transform", drag.transform_signal())
                        .global_event(clone!(state, drag => move |evt: events::PointerMove| {
                            if evt.is_primary() {
                                state.on_pointer_move(&drag, evt.x(), evt.y());
                            }
                        }))
                        .global_event(clone!(state, drag => move |evt: events::PointerUp| {
                            if evt.is_primary() {
                                state.on_pointer_up(&drag, evt.x(), evt.y());
                            }
                        }))
                        .global_event(clone!(state => move |evt:events::PointerCancel| {
                            if evt.is_primary() {
                                state.stop_drag();
                            }
                        }))
                        .child(render_asset_card(
                            &asset,
                            AssetCardConfig {
                                bottom_indicator: AssetCardBottomIndicator::Author,
                                dense: true,
                                ..Default::default()
                            }
                        ))
                    })
                })
            })))
            .child_signal(state.play_asset.signal_cloned().map(clone!(state => move|play_asset| {
                play_asset.map(|(asset_id, settings)| {
                    let close = clone!(state => move || {
                        state.play_asset.set(None);
                    });
                    PlayerPopup::new(
                        asset_id,
                        None,
                        None,
                        settings,
                        PreviewPopupCallbacks::new(close)
                    ).render(Some("player"))
                })
            })))
        })
    }

    fn render_asset(self: &Rc<Self>, jig: &Rc<JigResponse>) -> Dom {
        let state = self;
        let asset: Asset = (**jig).clone().into();
        let asset_id = asset.id();
        html!("div", {
            .prop("slot", "results")
            .style("cursor", "grab")
            .style("touch-action", "none")
            .style("user-select", "none")
            .style_signal("filter", state.drag.signal_ref(clone!(jig => move |drag| {
                match drag {
                    Some(drag) if drag.data.unwrap_jig().id == jig.id => "grayscale(100%) opacity(0.5)",
                    _ => "none",
                }
            })))
            .event_with_options(
                &EventOptions::bubbles(),
                clone!(state, jig => move |evt: events::PointerDown| {
                    if evt.is_primary() {
                        let elem = evt.dyn_target().unwrap_ji();
                        state.on_pointer_down(&elem, evt.x(), evt.y(), &jig);
                    }
                })
            )
            .child(render_asset_card(
                &asset,
                AssetCardConfig {
                    bottom_indicator: AssetCardBottomIndicator::Author,
                    dense: true,
                    menu: Some(Rc::new(clone!(state => move || {
                        html!("menu-kebab", {
                            .prop("slot", "menu")
                            .children(&mut [
                                html!("menu-line", {
                                    .prop("icon", "jig-play")
                                    .event(clone!(state => move |_: events::Click| {
                                        let settings = match asset_id {
                                            AssetId::JigId(_) => AssetPlayerOptions::Jig(JigPlayerOptions::default()),
                                            AssetId::ResourceId(_) => todo!(),
                                            AssetId::CourseId(_) => unimplemented!(),
                                            AssetId::ProDevId(_) => unimplemented!(),
                                        };
                                        state.play_asset.set(Some((asset_id, settings)));
                                    }))
                                })
                            ])
                        })
                    }))),
                    ..Default::default()
                },
            ))
        })
    }

    fn all_loaded_signal(self: &Rc<Self>) -> impl Signal<Item = bool> {
        map_ref! {
            let list_len = self.search_results.signal_vec_cloned().len(),
            let total = self.total_jig_count.signal() => move {
                *list_len >= *total as usize
            }
        }
    }
}
