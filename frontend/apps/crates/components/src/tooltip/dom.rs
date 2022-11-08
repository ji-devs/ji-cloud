use dominator::{clone, html, Dom, DomBuilder};
use futures_signals::signal::SignalExt;
use std::rc::Rc;
use utils::prelude::*;

use super::state::*;
use web_sys::{DomRect, HtmlElement};

//TODO - move all these to new style
pub fn render(state: Rc<State>) -> Dom {
    render_mixin(state, |dom: DomBuilder<HtmlElement>| dom)
}

pub fn render_mixin<F>(state: Rc<State>, mixin: F) -> Dom
where
    F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
{
    match &state.target {
        TooltipTarget::Element(elem, move_strategy) => match &state.data {
            TooltipData::Error(data) => {
                let TooltipError {
                    target_anchor,
                    content_anchor,
                    body,
                    max_width,
                    ..
                } = &**data;
                html!("overlay-tooltip-error", {
                    .apply(mixin)
                    .text(body)

                    .apply_if(state.container.is_some(), |dom| {
                        dom.prop("container", state.container.as_ref().unwrap_ji().as_value())
                    })
                    .prop("target", elem)
                    .prop("targetAnchor", target_anchor.as_str())
                    .prop("contentAnchor", content_anchor.as_str())
                    .prop("strategy", move_strategy.as_str())
                    .apply_if(max_width.is_some(), |dom| {
                        dom.prop("maxWidth", max_width.unwrap_ji())
                    })
                    .event(clone!(data => move |_evt:events::Close| {
                        if let Some(on_close) = data.callbacks.on_close.as_ref() {
                            (on_close) ();
                        }
                    }))
                })
            }

            TooltipData::Confirm(data) => {
                let TooltipConfirm {
                    target_anchor,
                    content_anchor,
                    header,
                    confirm_label,
                    max_width,
                    cancel_label,
                    ..
                } = &**data;
                html!("overlay-tooltip-confirm", {
                    .apply(mixin)
                    .prop("header", header)
                    .prop("confirmLabel", confirm_label)
                    .prop("cancelLabel", cancel_label)
                    .apply_if(max_width.is_some(), |dom| {
                        dom.prop("maxWidth", max_width.unwrap_ji())
                    })
                    .apply_if(state.container.is_some(), |dom| {
                        dom.prop("container", state.container.as_ref().unwrap_ji().as_value())
                    })
                    .prop("target", elem)
                    .prop("targetAnchor", target_anchor.as_str())
                    .prop("contentAnchor", content_anchor.as_str())
                    .prop("strategy", move_strategy.as_str())
                    .event(clone!(data => move |_evt:events::Accept| {
                        if let Some(on_confirm) = data.callbacks.on_confirm.as_ref() {
                            (on_confirm) ();
                        }
                    }))
                    .event(clone!(data => move |_evt:events::Close| {
                        if let Some(on_cancel) = data.callbacks.on_cancel.as_ref() {
                            (on_cancel) ();
                        }
                    }))
                })
            }

            TooltipData::Bubble(data) => {
                let TooltipBubble {
                    target_anchor,
                    content_anchor,
                    body,
                    max_width,
                    ..
                } = &**data;
                html!("overlay-tooltip-bubble", {
                    .apply(mixin)
                    .text(body)
                    .apply_if(state.container.is_some(), |dom| {
                        dom.prop("container", state.container.as_ref().unwrap_ji().as_value())
                    })
                    .prop("target", elem)
                    .prop("targetAnchor", target_anchor.as_str())
                    .prop("contentAnchor", content_anchor.as_str())
                    .prop("strategy", move_strategy.as_str())
                    .apply_if(max_width.is_some(), |dom| {
                        dom.prop("maxWidth", max_width.unwrap_ji())
                    })
                })
            }
        },

        //This really means "in the canvas area" too...
        TooltipTarget::NormalizedBounds(bounds, move_strategy) => {
            match &state.data {
                TooltipData::Bubble(data) => {
                    let TooltipBubble {
                        target_anchor,
                        content_anchor,
                        body,
                        max_width,
                        ..
                    } = &**data;
                    html!("overlay-tooltip-bubble", {
                        .apply(mixin)
                        .text(body)
                        .prop_signal("target", bounds.denormalize_fixed_signal().map(|bounds| {
                            let rect:DomRect = bounds.into();
                            log::info!("{:?}", bounds);
                            rect
                        }))
                        /*
                        .style_signal("transform", resize_info_signal().map(|resize_info| {
                            format!("scale({})", resize_info.scale)
                        }))
                        */
                        .apply_if(state.container.is_some(), |dom| {
                            dom.prop("container", state.container.as_ref().unwrap_ji().as_value())
                        })
                        .prop("targetAnchor", target_anchor.as_str())
                        .prop("contentAnchor", content_anchor.as_str())
                        .prop("strategy", move_strategy.as_str())
                        .apply_if(max_width.is_some(), |dom| {
                            dom.prop("maxWidth", max_width.unwrap_ji())
                        })
                    })
                }
                _ => {
                    unimplemented!("don't know how to render other tooltips here!")
                }
            }
        }
    }
}
