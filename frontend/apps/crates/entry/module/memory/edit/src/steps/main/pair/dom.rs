use dominator::{html, Dom, clone, with_node};
use std::rc::Rc;
use std::cell::RefCell;
use utils::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;
use js_sys::Reflect;
use futures_signals::{
    map_ref,
    signal::{self, Mutable, ReadOnlyMutable, SignalExt},
    signal_vec::SignalVecExt,
};
use dominator_helpers::signals::EitherSignal;
use components::image::search::types::*;
use components::tooltip::{
    state::{State as TooltipState, TooltipData, TooltipTarget, TooltipConfirm, MoveStrategy, Placement},
    callbacks::TooltipConfirmCallbacks
};
use super::state::*;
use super::card::dom::render as render_card;
use crate::steps::state::Step;

pub fn render(state: Rc<MainPair>) -> Dom {

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
                    .event(clone!(state => move |evt:events::Click| {

                        let tooltip = Rc::new(TooltipState::new(
                            TooltipTarget::Element(
                                elem.clone(), 
                                MoveStrategy::Destroy
                            ),

                            TooltipData::Confirm(Rc::new(TooltipConfirm {
                                placement: Placement::Right, 
                                slot: None,
                                max_width: Some(180.0),
                                header: crate::strings::confirm::STR_DELETE_PAIR_HEADER.to_string(),
                                confirm_label: crate::strings::confirm::STR_DELETE_PAIR_CONFIRM.to_string(),
                                cancel_label: crate::strings::confirm::STR_DELETE_PAIR_CANCEL.to_string(),
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


