use super::state::*;
use dominator::{html, Dom};
use futures_signals::signal_vec::SignalVecExt;
use std::rc::Rc;
use utils::prelude::*;

impl OverlayContainer {
    pub fn render(self: Rc<Self>, slot: Option<&str>) -> Dom {
        html!("overlay-container", {
            .apply_if(slot.is_some(), |dom| {
                dom.property("slot", slot.unwrap_ji())
            })
            .children_signal_vec(
                OVERLAY_MAP.with(|m| m
                    .entries_cloned()
                    .map(|(_, f)| f())
                )
            )
        })
    }
}
