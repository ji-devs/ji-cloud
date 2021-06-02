use dominator::{Dom, html, clone};
use futures_signals::signal_vec::SignalVecExt;
use std::rc::Rc;
use utils::prelude::*;
use futures_signals::signal::SignalExt;
use crate::module::history::state::HistoryState;
use web_sys::HtmlElement;
use super::state::*;

pub fn render(state: Rc<State>) -> Dom {
    match &state.target {
        TooltipTarget::Element(elem, move_strategy) => {
            match &state.data {
                TooltipData::Error(data) => {
                    let TooltipError {placement, slot, body, max_width, ..} = &**data;
                    html!("tooltip-error", {
                        .text(&body)
                        .apply_if(slot.is_some(), |dom| dom.property("slot", slot.as_ref().unwrap_ji()))
                        .property("target", elem)
                        .property("placement", placement.as_str())
                        .property("moveStrategy", move_strategy.as_str())
                        .apply_if(max_width.is_some(), |dom| {
                            dom.property("maxWidth", max_width.unwrap_ji())
                        })
                        .event(clone!(data => move |evt:events::Close| {
                            if let Some(on_close) = data.callbacks.on_close.as_ref() {
                                (on_close) ();
                            }
                        }))
                    })
                },

                TooltipData::Confirm(data) => {
                    let TooltipConfirm {placement, slot, header, confirm_label, max_width, cancel_label, ..} = &**data;
                    html!("tooltip-confirm", {
                        .apply_if(slot.is_some(), |dom| dom.property("slot", slot.as_ref().unwrap_ji()))
                        .property("header", header)
                        .property("confirmLabel", confirm_label)
                        .property("cancelLabel", cancel_label)
                        .property("moveStrategy", move_strategy.as_str())
                        .apply_if(max_width.is_some(), |dom| {
                            dom.property("maxWidth", max_width.unwrap_ji())
                        })
                        .property("target", elem)
                        .property("placement", placement.as_str())
                        .event(clone!(data => move |evt:events::Accept| {
                            if let Some(on_confirm) = data.callbacks.on_confirm.as_ref() {
                                (on_confirm) ();
                            }
                        }))
                        .event(clone!(data => move |evt:events::Close| {
                            if let Some(on_cancel) = data.callbacks.on_cancel.as_ref() {
                                (on_cancel) ();
                            }
                        }))
                    })
                },

                _ => {
                    unimplemented!("don't know how to render other tooltips here!")
                }
            }
        },
        TooltipTarget::NormalizedBounds(bounds) => {
            match &state.data {
                _ => {
                    unimplemented!("TODO")
                }
            }
        }
    }
}

