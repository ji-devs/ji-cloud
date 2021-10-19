use super::state::*;
use std::rc::Rc;
use dominator::{Dom, html, clone};
use futures_signals::{
    signal::{Signal, SignalExt, Mutable},
    signal_vec::{SignalVecExt},
    signal_map::{MutableBTreeMap, SignalMap, SignalMapExt}
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