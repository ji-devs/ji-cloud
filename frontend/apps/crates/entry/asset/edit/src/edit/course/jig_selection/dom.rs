use std::rc::Rc;

use dominator::{clone, html, with_node, Dom, EventOptions};
use futures_signals::signal_vec::SignalVecExt;
use shared::domain::jig::JigResponse;
use utils::events;
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
            .children_signal_vec(state.jigs.signal_vec_cloned().map(clone!(state => move|jig| {
                state.render_jig(&jig, vec![
                    html!("button", {
                        .text("X")
                        .event(clone!(state, jig => move |_: events::Click| {
                            state.remove_jig(&jig.id);
                        }))
                    }),
                    html!("button", {
                        .text("↥")
                        .event(clone!(state, jig => move |_: events::Click| {
                            state.move_up_jig(&jig.id);
                        }))
                    }),
                    html!("button", {
                        .text("↧")
                        .event(clone!(state, jig => move |_: events::Click| {
                            state.move_down_jig(&jig.id);
                        }))
                    }),
                ])
            })))
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
        })
    }

    fn render_jig(self: &Rc<Self>, jig: &JigResponse, actions: Vec<Dom>) -> Dom {
        html!("p", {
            .text(&jig.jig_data.display_name)
            .child(html!("br"))
            .text("by: ")
            .text(&jig.author_name.clone().unwrap_or_default())
            .children(actions)
        })
    }
}
