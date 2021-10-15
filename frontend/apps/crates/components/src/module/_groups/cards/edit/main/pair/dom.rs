use dominator::{clone, html, with_node, Dom};
use std::rc::Rc;

use utils::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

use futures_signals::signal::SignalExt;

use super::card::dom::render as render_card;
use super::state::*;
use crate::{
    module::_groups::cards::edit::{state::*, strings},
    tooltip::{
        callbacks::TooltipConfirmCallbacks,
        state::{
            Anchor, ContentAnchor, MoveStrategy, State as TooltipState, TooltipConfirm,
            TooltipData, TooltipTarget,
        },
    },
};
use shared::domain::jig::module::body::_groups::cards::Step;

pub fn render<RawData: RawDataExt, E: ExtraExt>(state: Rc<MainPair<RawData, E>>) -> Dom {
    if state.step == Step::One {
        html!("main-card-pair", {
            .property("hoverable", true)
            .property_signal("hoverLock", state.base.tooltips.delete.signal_ref(|x| x.is_some()))
            .property_signal("index", state.index.signal().map(|x| {
                JsValue::from_f64(x.unwrap_or_default() as f64)
            }))
            .child(render_card(state.left.clone()))
            .child(render_card(state.right.clone()))
            .child(html!("button-icon" => HtmlElement, {
                .property("slot", "close")
                .property("icon", "circle-x-blue")
                .with_node!(elem => {
                    .event(clone!(state => move |_evt:events::Click| {

                        let tooltip = Rc::new(TooltipState::new(
                            TooltipTarget::Element(
                                elem.clone(),
                                MoveStrategy::Destroy
                            ),

                            TooltipData::Confirm(Rc::new(TooltipConfirm {

                                target_anchor: Anchor::MiddleRight,
                                content_anchor: ContentAnchor::OppositeH,
                                max_width: Some(180.0),
                                header: strings::confirm::STR_DELETE_PAIR_HEADER.to_string(),
                                confirm_label: strings::confirm::STR_DELETE_PAIR_CONFIRM.to_string(),
                                cancel_label: strings::confirm::STR_DELETE_PAIR_CANCEL.to_string(),
                                callbacks: TooltipConfirmCallbacks::new(
                                    Some(clone!(state => move || {
                                        state.delete_pair(state.index.get().unwrap_or_default());
                                        state.base.tooltips.delete.set(None);
                                    })),
                                    Some(clone!(state => move || {
                                        state.base.tooltips.delete.set(None);
                                    }))
                                )
                            }))
                        ));
                        state.base.tooltips.delete.set(Some(tooltip));
                    }))
                })
            }))
        })
    } else {
        html!("main-card-pair", {
            .property("hoverable", false)
            .property_signal("index", state.index.signal().map(|x| {
                JsValue::from_f64(x.unwrap_or_default() as f64)
            }))
            .child(render_card(state.left.clone()))
            .child(render_card(state.right.clone()))
        })
    }
}
