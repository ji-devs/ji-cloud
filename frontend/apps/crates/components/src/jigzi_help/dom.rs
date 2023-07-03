use std::rc::Rc;

use dominator::{clone, html, with_node, Dom};
use futures_signals::{map_ref, signal::SignalExt};
use utils::events;
use web_sys::HtmlElement;

use crate::overlay::handle::OverlayHandle;

use super::state::JigziHelp;

const MARGIN_X: i32 = 20;
const STR_NO_SHOW_AGAIN: &str = "I don't want help";

impl JigziHelp {
    pub fn render(
        self: Rc<Self>,
        slot: Option<&'static str>,
        get_action: Rc<Option<impl Fn() -> Dom + 'static>>,
    ) -> Dom {
        let state = self;
        state.show_info_tooltip_delayed();

        let show_tooltip_signal = map_ref! {
            let show_info_tooltip = state.show_info_tooltip.signal_cloned(),
            let permanently_closed = state.permanently_closed.signal_cloned()
                => {
                    *show_info_tooltip && !*permanently_closed
                }
        };

        html!("jigzi-help", {
            .apply(move |dom| {
                match slot {
                    Some(slot) => dom.prop("slot", slot),
                    None => dom,
                }
            })
            .event(clone!(state => move |_: events::Click| {
                state.permanently_closed.set(false);
                state.show_info_tooltip.set(true);
            }))
            .child_signal(show_tooltip_signal.map(clone!(state, get_action => move |show_tooltip| {
                match show_tooltip {
                    false => None,
                    true => Some(
                        html!("empty-fragment" => HtmlElement, {
                            .with_node!(elem => {
                                .apply(OverlayHandle::lifecycle(
                                    clone!(state, get_action => move || {
                                        html!("overlay-tooltip-info", {
                                            .prop("marginX", MARGIN_X)
                                            .prop("target", &elem)
                                            .attr("targetAnchor", "br")
                                            .attr("contentAnchor", "oppositeV")
                                            .prop("title", &state.title)
                                            .prop("body", &state.body)
                                            .prop("closeable", true)
                                            .prop("strategy", "track")
                                            .event(clone!(state => move |_evt: events::Close| {
                                                state.show_info_tooltip.set(false);
                                            }))
                                            .apply(clone!(get_action => move |dom| {
                                                match &*get_action {
                                                    Some(get_action) => {
                                                        let child = get_action();
                                                        dom.child(html!("empty-fragment", {
                                                            .prop("slot", "actions")
                                                            .child(child)
                                                        }))
                                                    },
                                                    None => dom
                                                }
                                            }))
                                            .child(html!("button-rect", {
                                                .prop("slot", "actions")
                                                .prop("kind", "text")
                                                .prop("color", "green")
                                                .style("margin-left", "auto")
                                                .text(STR_NO_SHOW_AGAIN)
                                                .event(clone!(state => move |_evt: events::Click| {
                                                    state.permanently_close();
                                                }))
                                            }))
                                        })
                                    })
                                ))
                            })
                        })
                    ),
                }
            })))
        })
    }
}
