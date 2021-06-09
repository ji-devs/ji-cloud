use dominator::{DomBuilder, Dom, html, clone};
use futures_signals::signal_vec::SignalVecExt;
use std::rc::Rc;
use utils::{prelude::*, resize::resize_info_signal};
use futures_signals::signal::SignalExt;
use crate::module::history::state::HistoryState;
use super::state::*;
use web_sys::{HtmlElement, Element, DomRect};

pub fn render(state: Rc<State>) -> Dom {
    render_mixin(state, |dom:DomBuilder<HtmlElement>| dom)
}

pub fn render_mixin<F>(state: Rc<State>, mixin: F) -> Dom 
    where
        F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,

{
    match &state.target {
        TooltipTarget::Element(elem, move_strategy) => {
            match &state.data {
                TooltipData::Error(data) => {
                    let TooltipError {placement, slot, body, max_width, ..} = &**data;
                    html!("tooltip-error", {
                        .apply(|dom| mixin(dom))
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
                        .apply(|dom| mixin(dom))
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

                TooltipData::Bubble(data) => {
                    let TooltipBubble {placement, slot, body, max_width, ..} = &**data;
                    html!("tooltip-bubble", {
                        .apply(|dom| mixin(dom))
                        .text(&body)
                        .apply_if(slot.is_some(), |dom| dom.property("slot", slot.as_ref().unwrap_ji()))
                        .property("target", elem)
                        .property("placement", placement.as_str())
                        .property("moveStrategy", move_strategy.as_str())
                        .apply_if(max_width.is_some(), |dom| {
                            dom.property("maxWidth", max_width.unwrap_ji())
                        })
                    })
                },
                _ => {
                    unimplemented!("don't know how to render other tooltips here!")
                }
            }
        },

        //This really means "in the canvas area" too...
        TooltipTarget::NormalizedBounds(bounds, move_strategy) => {
            match &state.data {
                TooltipData::Bubble(data) => {
                    let TooltipBubble {placement, slot, body, max_width, ..} = &**data;
                    html!("tooltip-bubble", {
                        .apply(|dom| mixin(dom))
                        .text(&body)
                        .apply_if(slot.is_some(), |dom| dom.property("slot", slot.as_ref().unwrap_ji()))
                        .property_signal("target", bounds.denormalize_fixed_signal().map(|bounds| {
                            let rect:DomRect = bounds.into();
                            rect
                        }))
                        /*
                        .style_signal("transform", resize_info_signal().map(|resize_info| {
                            format!("scale({})", resize_info.scale)
                        }))
                        */
                        .property("placement", placement.as_str())
                        .property("moveStrategy", move_strategy.as_str())
                        .apply_if(max_width.is_some(), |dom| {
                            dom.property("maxWidth", max_width.unwrap_ji())
                        })
                    })
                },
                _ => {
                    unimplemented!("don't know how to render other tooltips here!")
                }
            }
        }
    }
}

