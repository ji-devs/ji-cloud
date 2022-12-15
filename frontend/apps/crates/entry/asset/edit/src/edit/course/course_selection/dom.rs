use std::rc::Rc;

use components::module::_common::thumbnail::{ModuleThumbnail, ThumbnailFallback};
use dominator::{clone, html, with_node, Dom, EventOptions};
use futures_signals::signal_vec::SignalVecExt;
use shared::domain::{asset::DraftOrLive, jig::JigResponse};
use utils::{events, unwrap::UnwrapJiExt};
use web_sys::HtmlInputElement;

use super::state::CourseSelection;

impl CourseSelection {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;
        html!("div", {
            .style("max-height", "100vh")
            .style("overflow", "auto")
            .prop("slot", "main")
            .children(&mut [
                html!("hr"),
                html!("h4", {
                    .text("Add new jigs")
                }),
                html!("form", {
                    .event_with_options(
                        &EventOptions::preventable(),
                        clone!(state => move|e: events::Submit| {
                            e.prevent_default();
                            state.search();
                        })
                    )
                    .children(&mut [
                        html!("input" => HtmlInputElement, {
                            .style("width", "400px")
                            .with_node!(elem => {
                                .event(clone!(state => move|_: events::Input| {
                                    let value = elem.value();
                                    *state.input.borrow_mut() = value;
                                }))
                            })
                        }),
                        html!("button", {
                            .prop("type", "submit")
                            .text("Search")
                        })
                    ])
                })
            ])
            .child(html!("div", {
                .style("display", "grid")
                .style("grid-template-columns", "repeat(auto-fill, 280px)")
                .style("gap", "20px")
                .style("padding", "20px")
                .children_signal_vec(state.search_results.signal_vec_cloned().map(clone!(state => move |jig| {
                    state.render_asset(&jig)
                })))
            }))
            .child_signal(state.drag.signal_ref(clone!(state => move|drag| {
                drag.as_ref().map(|drag| {
                    let asset = &drag.data;

                    html!("asset-card", {
                        .style("position", "fixed")
                        .style("top", "0")
                        .style("left", "0")
                        .style("z-index", "1")
                        .style("cursor", "grabbing")
                        .style("touch-action", "none")
                        .style("user-select", "none")
                        .style("pointer-events", "none")
                        .prop("title", asset.display_name())
                        .prop("playedCount", asset.plays())
                        .prop("likedCount", asset.likes())
                        .prop("author", asset.author_name().clone().unwrap_or_default())
                        .prop("language", asset.language())
                        .prop("kind", "jig")
                        .child(
                            ModuleThumbnail::new(
                                asset.id(),
                                asset.cover().cloned(),
                                ThumbnailFallback::Asset,
                                DraftOrLive::Live
                            ).render(Some("image"))
                        )
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
                    })
                })
            })))
        })
    }

    fn render_asset(self: &Rc<Self>, jig: &Rc<JigResponse>) -> Dom {
        let state = self;
        html!("asset-card", {
            .style("cursor", "grab")
            .style("touch-action", "none")
            .style("user-select", "none")
            .prop("title", &jig.jig_data.display_name)
            .prop("playedCount", jig.plays)
            .prop("likedCount", jig.likes)
            .prop("author", jig.author_name.clone().unwrap_or_default())
            .prop("language", &jig.jig_data.language)
            .prop("kind", "jig")
            .child(
                ModuleThumbnail::new(
                    jig.id.into(),
                    jig.jig_data.modules.first().cloned(),
                    ThumbnailFallback::Asset,
                    DraftOrLive::Live
                ).render(Some("image"))
            )
            .style_signal("filter", state.drag.signal_ref(clone!(jig => move |drag| {
                match drag {
                    Some(drag) if drag.data.unwrap_jig().id == jig.id => "grayscale(100%) opacity(0.5)",
                    _ => "none",
                }
            })))
            .event(clone!(state, jig => move |evt: events::PointerDown| {
                let elem = evt.dyn_target().unwrap_ji();
                state.on_pointer_down(&elem, evt.x(), evt.y(), &jig);
            }))
        })
    }
}
