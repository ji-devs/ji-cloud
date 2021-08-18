use dominator::{traits::AsStr, html, Dom, clone, DomBuilder};
use web_sys::HtmlElement;
use super::state::*;
use std::rc::Rc;
use utils::prelude::*;
use futures_signals::signal::SignalExt;
use wasm_bindgen::prelude::*; 

impl <T: AsStr + Clone + 'static, P: AsStr + 'static, L: AsStr + 'static> SimpleSelect <T, P, L> {
    pub fn render(state: Rc<Self>, slot: Option<&str>) -> Dom {
        Self::_render_mixin(
            state,
            slot,
            None::<fn(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>>,
        )
    }
    pub fn render_mixin<F>(state: Rc<Self>, slot: Option<&str>, mixin: F) -> Dom 
    where
        F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
    {
        Self::_render_mixin(state, slot, Some(mixin))
    }

    fn _render_mixin<F>(state: Rc<Self>, slot: Option<&str>, mixin: Option<F>) -> Dom
    where
        F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
    {
        html!("input-select", {
            .apply_if(slot.is_some(), |dom| {
                dom.property("slot", slot.unwrap_ji())
            })
            .property_signal("value", state.value.signal_cloned().map(|value| {
                match value {
                    None => JsValue::NULL,
                    Some(value) => JsValue::from_str(value.as_str())
                }
            }))
            .apply_if(state.label.is_some(), |dom| {
                dom.property("label", state.label.as_ref().unwrap_ji().as_str())
            })
            .apply_if(state.placeholder.is_some(), |dom| {
                dom.property("placeholder", state.placeholder.as_ref().unwrap_ji().as_str())
            })
            .children(state.values.iter().map(clone!(state => move |value| {
                html!("input-select-option", {
                    .text(value.as_str())
                    .event(clone!(state, value => move |evt:events::CustomSelectedChange| {
                        if evt.selected() {
                            state.value.set(Some(value.clone()));
                            if let Some(on_change) = state.on_change.as_ref() {
                                (on_change) (Some(value.as_str()));
                            }
                        }
                    }))
                })
            })))
            .apply_if(mixin.is_some(), |dom| {
                dom.apply(mixin.unwrap_ji())
            })
        })
    }
}

/*
                html!("input-select", {
                    .property("slot", "persona")
                    .property("label", STR_PERSONA_LABEL)
                    .property_signal("error", state.persona_error.signal())
                    .property("value", "Teacher")
                    .children(&mut [
                              html!("input-select-option", {
                                  .text("Teacher")
                              }),
                              html!("input-select-option", {
                                  .text("Student")
                              })
                    ])
                    .event(clone!(state => move |evt:events::Input| {
                        *state.persona.borrow_mut() = evt.value().and_then(|x| {
                            if x.is_empty() { None } else { Some(x) }
                        })
                    }))
                }),
                */
