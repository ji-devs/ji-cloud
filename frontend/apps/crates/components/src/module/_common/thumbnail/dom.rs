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
                    if let Some(module) = &state.module {
                        call_screenshot_service(state.jig_id, module.id, module.kind).await;
                    }
                }))
            }))
            .property("jigId", state.jig_id.0.to_string())
            .apply(clone!(state => move |dom| {
                match &state.module {
                    Some(module) => dom.property("moduleId", module.id.0.to_string()),
                    None => dom,
                }
            }))
            .apply(clone!(state => move |dom| {
                match &state.module {
                    Some(module) if !state.is_jig_fallback => {
                        dom.property("moduleKind", module.kind.as_str())
                    },
                    _ => dom
                }
            }))
        })
    }

    pub fn render_live(state: Rc<Self>, slot: Option<&str>) -> Dom {
        let mutable = Mutable::new(());

        let listener = Rc::new(RefCell::new(
            match &state.module {
                Some(_) => Some(
                    crate::firebase::listen_for_screenshot_updates(
                        &state.jig_id,
                        &state.module.clone().unwrap_ji().id,
                        clone!(mutable => move || {
                            mutable.set(());
                        }),
                    ),
                ),
                None => None
            }
        ));

        html!("img-module-screenshot", {
            .apply_if(slot.is_some(), |dom| {
                dom.property("slot", slot.unwrap_ji())
            })
            .event(clone!(state => move |_evt:events::ImageError| {
                spawn_local(clone!(state => async move {
                    if let Some(module) = &state.module {
                        call_screenshot_service(state.jig_id, module.id, module.kind).await;
                    }
                }))
            }))
            .property("jigId", state.jig_id.0.to_string())
            .apply(clone!(state => move |dom| {
                match &state.module {
                    None => dom,
                    Some(module) => {
                        dom.property_signal("moduleId", mutable.signal_ref(clone!(module => move |_| {
                            module.id.0.to_string()
                        })))
                    },
                }
            }))
            .property("cacheBust", true)
            .apply(clone!(state => move |dom| {
                match &state.module {
                    Some(module) if !state.is_jig_fallback => {
                        dom.property("moduleKind", module.kind.as_str())
                    },
                    _ => dom
                }
            }))
            //need to hold the listener for the lifetime of the element
            .after_removed(clone!(listener => move |_elem| {
                let _ = listener.borrow_mut().take();
            }))
        })
    }
}
