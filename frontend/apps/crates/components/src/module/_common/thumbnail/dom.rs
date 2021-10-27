use super::state::*;
use dominator::clone;
use dominator::{html, Dom};
use futures_signals::signal::Mutable;
use std::cell::RefCell;
use std::rc::Rc;
use utils::prelude::*;
use utils::screenshot::call_screenshot_service;
use wasm_bindgen_futures::spawn_local;

impl ModuleThumbnail {
    pub fn render(state: Rc<Self>, slot: Option<&str>) -> Dom {
        html!("img-module-screenshot", {
            .apply_if(slot.is_some(), |dom| {
                dom.property("slot", slot.unwrap_ji())
            })
            .event(clone!(state => move |_evt:events::ImageError| {
                spawn_local(clone!(state => async move {
                    call_screenshot_service(state.jig_id, state.module.id, state.module.kind).await;
                }))
            }))
            .property("jigId", state.jig_id.0.to_string())
            .property("moduleId", state.module.id.0.to_string())
            .apply_if(!state.is_jig_fallback, |dom| {
                dom.property("moduleKind", state.module.kind.as_str())
            })
        })
    }

    pub fn render_live(state: Rc<Self>, slot: Option<&str>) -> Dom {
        let module_id = Mutable::new(state.module.id);

        let listener = Rc::new(RefCell::new(Some(
            crate::firebase::listen_for_screenshot_updates(
                &state.jig_id,
                &state.module.id,
                clone!(module_id, state => move || {
                    module_id.set(state.module.id);
                }),
            ),
        )));

        html!("img-module-screenshot", {
            .apply_if(slot.is_some(), |dom| {
                dom.property("slot", slot.unwrap_ji())
            })
            .event(clone!(state => move |_evt:events::ImageError| {
                spawn_local(clone!(state => async move {
                    call_screenshot_service(state.jig_id, state.module.id, state.module.kind).await;
                }))
            }))
            .property("jigId", state.jig_id.0.to_string())
            .property_signal("moduleId", module_id.signal_ref(|id| id.0.to_string()))
            .property("cacheBust", true)
            .apply_if(!state.is_jig_fallback, |dom| {
                dom.property("moduleKind", state.module.kind.as_str())
            })
            //need to hold the listener for the lifetime of the element
            .after_removed(clone!(listener => move |_elem| {
                let _ = listener.borrow_mut().take();
            }))
        })
    }
}
