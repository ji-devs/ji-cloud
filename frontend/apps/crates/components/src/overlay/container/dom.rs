use super::state::*;
use std::rc::Rc;
use dominator::{Dom, html};
use futures_signals::{
    signal::SignalExt,
    signal_vec::SignalVecExt,
};
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