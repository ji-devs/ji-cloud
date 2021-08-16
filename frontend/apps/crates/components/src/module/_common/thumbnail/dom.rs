use dominator::{Dom, html};
use std::rc::Rc;
use utils::prelude::*;
use super::state::*;
use futures_signals::signal::{Mutable, Signal, SignalExt};
use dominator_helpers::signals::RcSignalFn;
use dominator::clone;
use std::cell::RefCell;

impl ModuleThumbnail {
    pub fn render(state: Rc<Self>, slot: Option<&str>) -> Dom {
        html!("img-module-screenshot", {
            .apply_if(slot.is_some(), |dom| {
                dom.property("slot", slot.unwrap_ji())
            })
            .property("jigId", state.jig_id.0.to_string())
            .property("moduleId", state.module.id.0.to_string())
            .apply_if(!state.is_jig_fallback, |dom| {
                dom.property("moduleKind", state.module.kind.as_str())
            })
        })
    }

    pub fn render_live(state: Rc<Self>, slot: Option<&str>) -> Dom {
        let module_id = Mutable::new(state.module.id.clone());

        let listener = Rc::new(RefCell::new(Some(crate::firebase::listen_for_screenshot_updates(
            &state.jig_id,
            &state.module.id,
            clone!(module_id, state => move || {
                module_id.set(state.module.id.clone());
            })
        ))));

        html!("img-module-screenshot", {
            .apply_if(slot.is_some(), |dom| {
                dom.property("slot", slot.unwrap_ji())
            })
            .property("jigId", state.jig_id.0.to_string())
            .property_signal("moduleId", module_id.signal_ref(|id| id.0.to_string())) 
            .property("cacheBust", true)
            .apply_if(!state.is_jig_fallback, |dom| {
                dom.property("moduleKind", state.module.kind.as_str())
            })
            //need to hold the listener for the lifetime of the element
            .after_removed(clone!(listener => move |elem| {
                let _ = listener.borrow_mut().take();
            }))
        })
    }
}
