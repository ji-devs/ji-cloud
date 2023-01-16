use std::rc::Rc;

use components::asset_card::{render_asset_card, AssetCardBottomIndicator, AssetCardConfig};
use dominator::{clone, html, Dom, EventOptions};
use futures_signals::signal_vec::SignalVecExt;
use shared::domain::{asset::Asset, jig::JigResponse};
use utils::{events, unwrap::UnwrapJiExt};

use super::state::CourseSelection;

impl CourseSelection {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;

        state.search();

        html!("asset-edit-course-selection", {
            .prop("slot", "main")
            .child(state.search_bar.render(Rc::new(clone!(state => move || {
                state.search();
            }))))
            .child(html!("home-search-results", {
                .prop("slot", "results")
                .prop_signal("jigCount", state.search_results.signal_vec_cloned().len())
                .prop_signal("query", state.search_bar.search_selected.query.signal_cloned())
                .child(html!("home-search-results-section", {
                    .prop("slot", "sections")
                    .prop("kind", "jig")
                    .prop_signal("resultsCount", state.search_results.signal_vec_cloned().len())
                    .children_signal_vec(state.search_results.signal_vec_cloned().map(clone!(state => move |jig| {
                        state.render_asset(&jig)
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
                            state.on_pointer_move(&drag, evt.x(), evt.y());
                        }))
                        .global_event(clone!(state, drag => move |evt: events::PointerUp| {
                            state.on_pointer_up(&drag, evt.x(), evt.y());
                        }))
                        .global_event(clone!(state => move |_:events::PointerCancel| {
                            state.stop_drag();
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
        })
    }

    fn render_asset(self: &Rc<Self>, jig: &Rc<JigResponse>) -> Dom {
        let state = self;
        let asset: Asset = (**jig).clone().into();
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
                    let elem = evt.dyn_target().unwrap_ji();
                    state.on_pointer_down(&elem, evt.x(), evt.y(), &jig);
                })
            )
            .child(render_asset_card(
                &asset,
                AssetCardConfig {
                    bottom_indicator: AssetCardBottomIndicator::Author,
                    dense: true,
                    menu: Some(Rc::new(move || {
                        html!("menu-kebab", {
                            .prop("slot", "menu")
                            .children(&mut [
                            ])
                        })
                    })),
                    ..Default::default()
                },
            ))
        })
    }
}
