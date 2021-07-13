use futures_signals::{
    map_ref,
    signal_vec::{SignalVecExt, SignalVec, MutableVec},
    signal::{Signal, SignalExt, Mutable, ReadOnlyMutable},
};

use std::rc::Rc;
use std::cell::RefCell;
use shared::domain::jig::module::body::{_groups::design::Trace as RawTrace, Transform};
use crate::transform::state::TransformState;
use dominator::clone;
use super::{
    draw::state::*,
    all::trace::state::*,
    callbacks::*,
};
use crate::traces::utils::TraceExt;
use utils::{
    prelude::*, 
    drag::Drag,
    resize::get_resize_info
};
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d};

pub struct TracesEdit 
{
    pub list: MutableVec<Rc<AllTrace>>,
    pub selected_index: Mutable<Option<usize>>,
    pub phase: Mutable<Phase>,
    pub callbacks: Callbacks,
}


#[derive(Clone)]
pub enum Phase {
    All,
    Draw(Rc<Draw>),
}


#[derive(Clone, Debug, Default)]
pub struct DebugOptions {
    pub start_in_phase_draw: bool, 
}

impl TracesEdit {
    pub fn to_raw(&self) -> Vec<RawTrace> {
        self.list
            .lock_ref()
            .iter()
            .map(|trace| trace.to_raw())
            .collect()
    }

    pub fn from_raw(raw:&[RawTrace], debug_opts:Option<DebugOptions>, callbacks: Callbacks) -> Rc<Self> {

        let debug_opts = debug_opts.unwrap_or_default();

        let _self = Rc::new(Self{
            list: MutableVec::new(),
            selected_index: Mutable::new(None),
            phase: Mutable::new(Phase::All),
            callbacks,
        });


        if raw.len() > 0 {
            let resize_info = get_resize_info();
            _self.list.lock_mut().replace_cloned( 
                        raw.
                            into_iter()
                            .map(|trace| {
                                Rc::new(AllTrace::new(trace.clone(), &resize_info))
                            })
                            .collect()
            );
        }

        if debug_opts.start_in_phase_draw {
            Self::start_draw(_self.clone(), None, None);
        }

        _self

    }

    pub fn get_current(&self) -> Option<Rc<AllTrace>> {
        self
            .selected_index
            .get_cloned()
            .and_then(|i| self.get(i))
    }


    pub fn get(&self, index: usize) -> Option<Rc<AllTrace>> {
        self.list.lock_ref().get(index).map(|x| x.clone())
    }

    pub fn selected_signal(&self, index: ReadOnlyMutable<Option<usize>>) -> impl Signal<Item = bool> {
        map_ref! {
            let index = index.signal(),
            let selected = self.selected_index.signal_cloned()
                => {
                    match (*index, *selected) {
                        (Some(index), Some(selected)) => {
                            index == selected
                        },
                        _ => false
                    }
                }
        }
    }

}

