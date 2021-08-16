use dominator::{Dom, html};
use std::rc::Rc;
use utils::prelude::*;
use super::state::*;
use futures_signals::signal::{Signal, SignalExt};
use dominator_helpers::signals::RcSignalFn;

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
}
