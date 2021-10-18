// OverlayContainer is expected to be instantiated only once per-app
// breaking that probably won't crash, but it will render the same
// children in both instances
//
// The overall idea is that we are misbehaving here, going outside
// the dom flow, using something like a global mutable event system
// but still using Dominator semantics to control it

use std::{cell::RefCell, rc::Rc, sync::atomic::{AtomicUsize, Ordering}};
use awsm_web::loaders::helpers::AsyncLoader;
use futures_signals::{
    signal::{Signal, SignalExt, Mutable},
    signal_map::{MutableBTreeMap, SignalMap, SignalMapExt}
};
use dominator::{Dom, clone};
use utils::prelude::*;


pub(super) static KEY_COUNTER:AtomicUsize = AtomicUsize::new(0);
thread_local! {
    pub(super) static OVERLAY_MAP:MutableBTreeMap<usize, Rc<dyn Fn() -> Dom>> = MutableBTreeMap::new();
}

pub struct OverlayContainer {
}

impl OverlayContainer {
    pub fn new() -> Rc<Self> {
        Rc::new(
            Self{
            }
        )
    } 
}