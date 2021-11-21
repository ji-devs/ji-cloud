use super::state::*;
use super::actions::*;
use std::rc::Rc;
use utils::{prelude::*, resize::{ResizeInfo, resize_info_signal}};
use crate::base::styles;
use futures_signals::signal::{Mutable, SignalExt};
use dominator::{html, Dom, clone, with_node};
use dominator_helpers::signals::{DefaultSignal, DomRectSignal};
use awsm_web::dom::*;
use wasm_bindgen::JsCast;


impl TalkType {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;

        html!("div", {
            .class(&*styles::FULL_STAGE)
            .children_signal_vec(
                resize_info_signal().map(clone!(state => move |resize_info| {
                    state.items
                        .iter()
                        .map(|item| item.clone().render_text_input(state.clone(), &resize_info))
                        .collect()
                }))
                .to_signal_vec()
            )
        })
    }
}

impl TalkTypeItem {
    pub fn render_text_input(
        self: Rc<Self>,
        parent: Rc<TalkType>,
        resize_info: &ResizeInfo,
    ) -> Dom {
        let state = self;
        let bounds = state.bounds.denormalize(resize_info);


        html!("legacy-input-fit", {
            .property("y", bounds.y) 
            .property("x", bounds.x) 
            .property("width", bounds.width) 
            .property("height", bounds.height) 
            .property_signal("value", state.value.signal_cloned()) 
            .event(clone!(state => move |evt:events::CustomInput| {
                state.value.set_neq(evt.value())
            }))
            .event(clone!(state, parent => move |evt:events::Enter| {
                state.clone().evaluate(parent.clone())
            }))
        })
    }
}
