use dominator::{Dom, html, clone};
use futures_signals::signal_vec::SignalVecExt;
use std::rc::Rc;
use utils::prelude::*;
use futures_signals::signal::SignalExt;
use crate::module::history::state::HistoryState;
use web_sys::HtmlElement;
use super::state::*;

pub fn render(state: Rc<State>) -> Dom {
    match state.target.clone() {
        TooltipTarget::Element(elem, move_strategy) => {
            match state.data.clone() {
                TooltipData::Error(data) => {
                    let TooltipError {placement, slot, body, on_close, max_width } = data;
                    html!("tooltip-error", {
                        .text(&body)
                        .apply_if(slot.is_some(), |dom| dom.property("slot", slot.unwrap_ji()))
                        .property("target", elem)
                        .property("placement", placement.as_str())
                        .property("moveStrategy", move_strategy.as_str())
                        .apply_if(max_width.is_some(), |dom| {
                            dom.property("maxWidth", max_width.unwrap_ji())
                        })
                        .event(move |evt:events::Close| {
                            if let Some(on_close) = &on_close {
                                on_close();
                            }
                        })
                    })
                },

                TooltipData::Confirm(data) => {
                    let TooltipConfirm {placement, slot, header, confirm_label, max_width, cancel_label, on_confirm, on_cancel} = data;
                    html!("tooltip-confirm", {
                        .apply_if(slot.is_some(), |dom| dom.property("slot", slot.unwrap_ji()))
                        .property("header", header)
                        .property("confirmLabel", confirm_label)
                        .property("cancelLabel", cancel_label)
                        .property("moveStrategy", move_strategy.as_str())
                        .apply_if(max_width.is_some(), |dom| {
                            dom.property("maxWidth", max_width.unwrap_ji())
                        })
                        .property("target", elem)
                        .property("placement", placement.as_str())
                        .event(clone!(on_confirm => move |evt:events::Accept| {
                            on_confirm();
                        }))
                        .event(clone!(on_cancel => move |evt:events::Close| {
                            on_cancel();
                        }))
                    })
                },

                _ => {
                    unimplemented!("don't know how to render other tooltips here!")
                }
            }
        },
        TooltipTarget::NormalizedBounds(bounds) => {
            match state.data.clone() {
                _ => {
                    unimplemented!("TODO")
                }
            }
        }
    }
}

