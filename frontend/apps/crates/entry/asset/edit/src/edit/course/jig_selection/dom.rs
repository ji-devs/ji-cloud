use std::rc::Rc;

use dominator::{clone, html, with_node, Dom, EventOptions};
use futures_signals::signal_vec::SignalVecExt;
use shared::domain::jig::JigResponse;
use utils::{events, unwrap::UnwrapJiExt};
use web_sys::HtmlInputElement;

use super::state::JigSelection;

impl JigSelection {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;
        state.load_course();
        html!("div", {
            .style("max-height", "100vh")
            .style("overflow", "auto")
            .prop("slot", "main")
            // .children_signal_vec(state.asset_edit_state.sidebar_spots.signal_vec_cloned().map(clone!(state => move|spot| {
            //     state.render_jig(&spot, vec![
            //         html!("button", {
            //             .text("X")
            //             .event(clone!(state, spot => move |_: events::Click| {
            //                 state.remove_jig(&jig.id);
            //             }))
            //         }),
            //         html!("button", {
            //             .text("↥")
            //             .event(clone!(state, spot => move |_: events::Click| {
            //                 state.move_up_jig(&jig.id);
            //             }))
            //         }),
            //         html!("button", {
            //             .text("↧")
            //             .event(clone!(state, spot => move |_: events::Click| {
            //                 state.move_down_jig(&jig.id);
            //             }))
            //         }),
            //     ])
            // })))
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
            .children_signal_vec(state.search_results.signal_vec_cloned().map(clone!(state => move |jig| {
                state.render_jig(&jig, vec![html!("button", {
                    .text("+")
                    .event(clone!(state, jig => move |_: events::Click| {
                        state.add_jig(Rc::clone(&jig));
                    }))
                })])
            })))
            .child_signal(state.drag.signal_ref(clone!(state => move|drag| {
                drag.as_ref().map(|drag| {
                    html!("input", {
                        .prop("slot", "dragged")
                        // .prop("path", &format!("entry/jig/modules/large/{}-hover.svg", state.kind.as_str()))
                        .style("position", "fixed")
                        .style("top", "0")
                        .style("left", "0")
                        .style("z-index", "1")
                        .style("cursor", "grabbing")
                        /* for elementFromPoint not to return the dragged element */
                        .style("pointer-events", "none")
                        .prop("readOnly", true)
                        // .prop("value", )
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

    fn render_jig(self: &Rc<Self>, jig: &JigResponse, actions: Vec<Dom>) -> Dom {
        let state = self;
        html!("p", {
            .text(&jig.jig_data.display_name)
            .child(html!("br"))
            .text("by: ")
            .text(&jig.author_name.clone().unwrap_or_default())
            .children(actions)
            .style("touch-action", "none")
            .style("user-select", "none")
            .event(clone!(state => move |evt: events::PointerDown| {
                let elem = evt.dyn_target().unwrap_ji();
                state.on_pointer_down(&elem, evt.x(), evt.y());
            }))
        })
    }
}
