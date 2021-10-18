use super::state::*;
use std::rc::Rc;
use dominator::{Dom, html, clone};
use futures_signals::{
    signal::{Signal, SignalExt, Mutable},
    signal_vec::{SignalVecExt},
    signal_map::{MutableBTreeMap, SignalMap, SignalMapExt}
};

impl OverlayContainer {
    pub fn render(self: Rc<Self>) -> Dom { 
        html!("overlay-container", {
            .children_signal_vec(
                OVERLAY_MAP.with(|m| m
                    .entries_cloned()
                    .map(|(_, f)| f())
                )
            )
        })
    } 
} 