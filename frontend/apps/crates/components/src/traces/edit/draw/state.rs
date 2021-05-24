use std::rc::Rc;
use std::cell::RefCell;
use shared::domain::jig::module::body::{Trace as RawTrace, Transform};
use crate::traces::trace::state::*;
use dominator::clone;
use super::menu::state::*;
use utils::{
    prelude::*, 
    drag::Drag,
    resize::get_resize_info
};
use futures_signals::{
    map_ref,
    signal_vec::{SignalVecExt, SignalVec, MutableVec},
    signal::{Signal, SignalExt, Mutable, ReadOnlyMutable},
};

pub struct Draw {
    pub trace: Trace,
    pub draw_points: Mutable<Vec<(f64, f64)>>, 
    pub display_trace: Mutable<bool>,
    pub drag: Mutable<Option<Drag>>,
    pub menu: Mutable<Option<Menu>>,
    pub on_finished: Box<dyn Fn(Option<Trace>)>
}

impl Draw {
    pub fn new(on_finished: impl Fn(Option<Trace>) + 'static) -> Self {


        let draw_points = Mutable::new(Vec::new());

        let menu:Mutable<Option<Menu>> = Mutable::new(None);

        Self {
            trace: Trace::new(None, Rc::new(Box::new(clone!(menu => move || {
                //this will trigger menu re-positioning
                if let Some(curr) = menu.get_cloned() {
                    menu.set(Some(curr));
                }
            })))),
            menu,
            drag: Mutable::new(None),
            draw_points,
            display_trace: Mutable::new(false),
            on_finished: Box::new(on_finished),
        }
    }

}
