use std::rc::Rc;

use dominator::{clone, html, Dom, with_node};
use futures_signals::{signal::SignalExt, map_ref};
use js_sys::Reflect;
use utils::events;
use wasm_bindgen::JsValue;
use web_sys::HtmlElement;

use crate::overlay::handle::OverlayHandle;

use super::state::JigziHelp;

const MARGIN_X: i32 = 20;

impl JigziHelp {
    pub fn render(self: Rc<Self>, slot: Option<&'static str>) -> Dom {
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
                    Some(slot) => dom.property("slot", slot),
                    None => dom,
                }
            })
            .event(clone!(state => move |_: events::Click| {
                state.permanently_closed.set(false);
                state.show_info_tooltip.set(true);
            }))
            .child_signal(show_tooltip_signal.map(clone!(state => move|show_tooltip| {
                match show_tooltip {
                    false => None,
                    true => Some(
                        html!("empty-fragment" => HtmlElement, {
                            .with_node!(elem => {
                                .apply(OverlayHandle::lifecycle(
                                    clone!(state => move || {
                                        html!("overlay-tooltip-info", {
                                            .property("marginX", MARGIN_X)
                                            .property("target", &elem)
                                            .attribute("targetAnchor", "br")
                                            .attribute("contentAnchor", "oppositeV")
                                            .property("title", &state.title)
                                            .property("body", &state.body)
                                            .property("showPermanentlyClose", !&state.show_id.is_empty())
                                            .property("closeable", true)
                                            .property("strategy", "track")
                                            .event(clone!(state => move |_evt: events::Close| {
                                                state.show_info_tooltip.set(false);
                                            }))
                                            .event(clone!(state => move |_evt: events::PermanentlyClose| {
                                                state.permanently_close();
                                            }))
                                            .after_inserted(move |elem| {
                                                let _ = Reflect::set(
                                                    &elem,
                                                    &JsValue::from_str("selfClosed"),
                                                    &JsValue::from_bool(false),
                                                );
                                            })
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
