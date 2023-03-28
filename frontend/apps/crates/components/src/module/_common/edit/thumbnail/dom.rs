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
    pub fn render(self: Rc<Self>, slot: Option<&str>) -> Dom {
        let state = self;
        html!("img-module-screenshot", {
            .apply_if(slot.is_some(), |dom| {
                dom.prop("slot", slot.unwrap_ji())
            })
            .event(clone!(state => move |_evt:events::ImageError| {
                state.on_image_load_error();
            }))
            .prop("jigId", state.asset_id.uuid().to_string())
            .apply_if(state.hight_res, |dom| {
                dom.prop("size", "full")
            })
            .apply(clone!(state => move |dom| {
                match &state.module {
                    Some(module) => dom.prop("moduleId", module.id.0.to_string()),
                    None => dom,
                }
            }))
            .apply(clone!(state => move |dom| {
                match &state.module {
                    Some(module) if state.fallback == ThumbnailFallback::Module => {
                        dom.prop("moduleKind", module.kind.as_str())
                    },
                    _ => dom
                }
            }))
        })
    }

    pub fn render_live(self: Rc<Self>, slot: Option<&str>) -> Dom {
        let state = self;
        let mutable = Mutable::new(());

        let listener = Rc::new(RefCell::new(state.module.as_ref().map(|_| {
            crate::firebase::listen_for_screenshot_updates(
                &state.asset_id,
                &state.module.clone().unwrap_ji().id,
                clone!(mutable => move || {
                    mutable.set(());
                }),
            )
        })));

        html!("img-module-screenshot", {
            .apply_if(slot.is_some(), |dom| {
                dom.prop("slot", slot.unwrap_ji())
            })
            .event(clone!(state => move |_evt: events::ImageError| {
                state.on_image_load_error();
            }))
            .prop("jigId", state.asset_id.uuid().to_string())
            .apply_if(state.hight_res, |dom| {
                dom.prop("size", "full")
            })
            .apply(clone!(state => move |dom| {
                match &state.module {
                    None => dom,
                    Some(module) => {
                        dom.prop_signal("moduleId", mutable.signal_ref(clone!(module => move |_| {
                            module.id.0.to_string()
                        })))
                    },
                }
            }))
            .prop("cacheBust", true)
            .apply(clone!(state => move |dom| {
                match &state.module {
                    Some(module) if state.fallback == ThumbnailFallback::Module => {
                        dom.prop("moduleKind", module.kind.as_str())
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

    fn on_image_load_error(self: &Rc<Self>) {
        let state = self;
        if let Some(module) = &state.module {
            // We need to ensure that the screenshot is only generated for activities which
            // have their content set, otherwise it will render a possible error page.
            if module.is_complete {
                spawn_local(clone!(state => async move {
                    // Don't need to clone module, we can fetch it out of state. Also,
                    // unwrapping is fine here as we've already validated that it is
                    // Some.
                    let module = state.module.as_ref().unwrap_ji();
                    call_screenshot_service(state.asset_id, module.id, module.kind, state.draft_or_live).await;
                }))
            } else {
                log::info!("not complete");
            }
        }
    }
}
