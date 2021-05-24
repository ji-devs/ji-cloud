use futures_signals::{
    map_ref,
    signal_vec::{SignalVecExt, SignalVec, MutableVec},
    signal::{Signal, SignalExt, Mutable, ReadOnlyMutable},
};

use std::rc::Rc;
use std::cell::RefCell;
use shared::domain::jig::module::body::{Trace as RawTrace, Transform};
use crate::transform::state::TransformState;
use dominator::clone;
use crate::traces::trace::state::Trace;
use super::draw::state::*;
use utils::{
    prelude::*, 
    drag::Drag,
    resize::get_resize_info
};
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d};

pub struct Edit 
{
    pub list: MutableVec<Rc<Trace>>,
    pub selected_index: Mutable<Option<usize>>,
    pub on_change: Option<Box<dyn Fn(Vec<RawTrace>)>>,
    pub on_change_cb: RefCell<Option<Rc<Box<dyn Fn()>>>>, 
    pub phase: Mutable<Phase>,
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

impl Edit {
    pub fn to_raw(&self) -> Vec<RawTrace> {
        self.list
            .lock_ref()
            .iter()
            .map(|trace| trace.to_raw())
            .collect()
    }

    pub fn new(raw:Option<&[RawTrace]>, debug_opts:Option<DebugOptions>, on_change: Option<impl Fn(Vec<RawTrace>) + 'static>) -> Rc<Self> {

        let debug_opts = debug_opts.unwrap_or_default();

        let _self = Rc::new(Self{
            list: MutableVec::new(),
            selected_index: Mutable::new(None),
            on_change: match on_change {
                //map doesn't work for som reason
                None => None,
                Some(f) => Some(Box::new(f))
            },
            phase: Mutable::new(Phase::All),
            on_change_cb: RefCell::new(None)
        });

        *_self.on_change_cb.borrow_mut() = Some(Rc::new(Box::new(clone!(_self => move || _self.call_change()))));


        if let Some(raw) = raw {
            _self.list.lock_mut().replace_cloned( 
                        raw.
                            into_iter()
                            .map(|trace| {
                                Rc::new(Trace::new(
                                    Some(trace.clone()),
                                    _self.on_change_cb.borrow().as_ref().unwrap_ji().clone()
                                ))
                            })
                            .collect()
            );
        }

        if debug_opts.start_in_phase_draw {
            Self::start_new_trace(_self.clone());
        }

        _self

    }

    pub fn get_current(&self) -> Option<Rc<Trace>> {
        self
            .selected_index
            .get_cloned()
            .and_then(|i| self.get(i))
    }


    pub fn get(&self, index: usize) -> Option<Rc<Trace>> {
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

