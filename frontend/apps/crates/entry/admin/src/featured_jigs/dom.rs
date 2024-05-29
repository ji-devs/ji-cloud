use components::{
    asset_card::{render_asset_card, AssetCardBottomIndicator, AssetCardConfig},
    module::_common::thumbnail::{ModuleThumbnail, ThumbnailFallback},
    player_popup::{PlayerPopup, PreviewPopupCallbacks},
};
use dominator::{clone, html, Dom, EventOptions};
use futures_signals::{
    map_ref,
    signal::{Signal, SignalExt},
    signal_vec::SignalVecExt,
};
use shared::domain::{
    asset::{Asset, DraftOrLive},
    jig::JigResponse,
};
use std::rc::Rc;

use super::state::*;

use utils::{asset::AssetPlayerOptions, events, toasts, unwrap::UnwrapJiExt};

const STR_LOAD_MORE: &str = "See more";

impl FeaturedJigs {
    pub fn render(self: &Rc<Self>) -> Dom {
        let state = self;

        state.search();
        state.load_featured();

        html!("div", {
            .style("display", "grid")
            .style("grid-template-columns", "180px 1fr")
            .child(html!("div", {
                .style("margin", "10px")
                .style("padding", "10px")
                .style("border-radius", "10px")
                .style("border", "solid 2px var(--main-blue)")
                .style_signal("border-style", state.dragging_over_drop_section.signal().map(|over| match over {
                    true => "dashed",
                    false => "solid",
                }))
                .event(clone!(state => move |_: events::CustomDragEnter| {
                    state.dragging_over_drop_section.set(true);
                }))
                .event(clone!(state => move |_: events::CustomDragLeave| {
                    state.dragging_over_drop_section.set(false);
                }))
                .event(clone!(state => move |e: events::CustomDrop| {
                    state.dragging_over_drop_section.set(false);
                    if let Some(data) = e.detail().as_string() {
                        let jig: JigResponse = serde_json::from_str(&data).unwrap();
                        let mut featured_jigs = state.featured_jigs.lock_mut();
                        if featured_jigs.iter().any(|j| j.id == jig.id) {
                            toasts::error("Jig already in featured list");
                        } else {
                            featured_jigs.push_cloned(jig);
                            state.update_featured();
                        }
                    }
                }))
                .child(html!("h4", {
                    .text("Drag in JIGs")
                }))
                .children_signal_vec(state.featured_jigs.signal_vec_cloned().enumerate().map(clone!(state => move |(i, jig)| {
                    html!("div", {
                        .style("margin-block", "30px")
                        .style("font-size", "12px")
                        .style("display", "-webkit-box")
                        .style("-webkit-line-clamp", "2")
                        .style("-webkit-box-orient", "vertical")
                        .style("overflow", "hidden")
                        .style("text-overflow", "ellipsis")
                        .child(html!("div", {
                           .style("display", "flex")
                           .children(&mut [
                                html!("button", {
                                    .prop_signal("disabled", i.signal().map(|i| match i {
                                        Some(i) => i == 0,
                                        None => true,
                                    }))
                                    .text("up")
                                    .event(clone!(state, i => move |_: events::Click| {
                                        if let Some(i) = i.get() {
                                            state.featured_jigs.lock_mut().swap(i, i-1);
                                            state.update_featured();
                                        }
                                    }))
                                }),
                                html!("button", {
                                    .prop_signal("disabled", map_ref! {
                                        let i = i.signal(),
                                        let len = state.featured_jigs.signal_vec_cloned().len() => match i {
                                            Some(i) => *i == len - 1,
                                            None => true,
                                        }
                                    })
                                    .text("down")
                                    .event(clone!(state, i => move |_: events::Click| {
                                        if let Some(i) = i.get() {
                                            state.featured_jigs.lock_mut().swap(i, i+1);
                                            state.update_featured();
                                        }
                                    }))
                                }),
                                html!("button", {
                                    .text("delete")
                                    .event(clone!(state, i => move |_: events::Click| {
                                        if let Some(i) = i.get() {
                                            state.featured_jigs.lock_mut().remove(i);
                                            state.update_featured();
                                        }
                                    }))
                                }),
                           ])
                        }))
                        .child(ModuleThumbnail::new(jig.id.into(), jig.jig_data.modules.get(0).cloned(), ThumbnailFallback::Asset, DraftOrLive::Live).render(None))
                        .text(&jig.jig_data.display_name)
                    })
                })))
            }))
            .child(state.render_search())
        })
    }

    fn render_search(self: &Rc<Self>) -> Dom {
        let state = self;

        let search_callback: Rc<dyn Fn()> = Rc::new(clone!(state => move || {
            state.next_page.set(0);
            state.search_results.lock_mut().clear();
            state.search();
        }));

        html!("div", {
            .style("padding", "50px")
            .style("background-color", "#d8e7fa")
            .child(state.search_bar.render_bar(Rc::clone(&search_callback)))
            .child(html!("home-search-results", {
                .prop_signal("jigCount", state.total_jig_count.signal())
                .prop_signal("query", state.active_query.signal_cloned())
                .child(html!("home-search-results-section", {
                    .prop("slot", "sections")
                    .prop("kind", "jig")
                    .prop("dense", true)
                    .prop_signal("resultsCount", state.total_jig_count.signal())
                    .children_signal_vec(state.search_results.signal_vec_cloned().map(clone!(state => move |jig| {
                        state.render_jig(&jig)
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
                    let jig = drag.data.clone();
                    html!("div", {
                        .style("position", "fixed")
                        .style("top", "0")
                        .style("left", "0")
                        .style("z-index", "1")
                        .style("cursor", "grabbing")
                        .style("touch-action", "none")
                        .style("user-select", "none")
                        .style("pointer-events", "none")
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
                            &Asset::Jig(jig),
                            AssetCardConfig {
                                bottom_indicator: AssetCardBottomIndicator::Author,
                                dense: true,
                                ..Default::default()
                            }
                        ))
                    })
                })
            })))
            .child_signal(state.play_jig.signal_cloned().map(clone!(state => move|play_jig| {
                play_jig.map(|jig_id| {
                    let close = clone!(state => move || {
                        state.play_jig.set(None);
                    });
                    PlayerPopup::new(
                        jig_id.into(),
                        None,
                        None,
                        AssetPlayerOptions::Jig(Default::default()),
                        PreviewPopupCallbacks::new(close)
                    ).render(Some("player"))
                })
            })))
        })
    }

    fn render_jig(self: &Rc<Self>, jig: &Rc<JigResponse>) -> Dom {
        let state = self;
        let jig_id = jig.id;
        let asset: Asset = (**jig).clone().into();
        html!("div", {
            .prop("slot", "results")
            .style("cursor", "grab")
            .style("touch-action", "none")
            .style("user-select", "none")
            .style_signal("filter", state.drag.signal_ref(clone!(jig => move |drag| {
                match drag {
                    Some(drag) if drag.data.id == jig.id => "grayscale(100%) opacity(0.5)",
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
                                        state.play_jig.set(Some(jig_id));
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
