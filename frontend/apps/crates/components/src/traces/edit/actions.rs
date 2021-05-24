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
use super::{
    state::*,
    draw::state::*
};
use crate::traces::trace::state::*;
use utils::{
    prelude::*, 
    drag::Drag,
    resize::get_resize_info
};


impl Edit {
    pub fn delete_index(&self, index: usize) {
        self.list.lock_mut().remove(index);
        self.call_change();
        /*
        self.get_history().push_modify(|game_data| {
            game_data.pairs.remove(pair_index);
        });
        */
    }


    pub fn select_index(&self, index:usize) {
        self.selected_index.set(Some(index));
    }


    pub fn deselect(&self) {
        self.selected_index.set(None);
    }

    pub fn start_new_trace(_self: Rc<Self>) {
        let draw = Draw::new(clone!(_self => move |trace| {
            //On finished
            if let Some(trace) = trace {
                let mut list = _self.list.lock_mut();
                list.push_cloned(Rc::new(Trace::new(
                    Some(trace.to_raw()),
                    _self.on_change_cb.borrow().as_ref().unwrap_ji().clone()
                )));

                _self.select_index(list.len()-1);

                _self.phase.set(Phase::All);


            }
        }));
        _self.phase.set(Phase::Draw(Rc::new(draw)));
    }
    // Internal - saving/history is done on the module level
    pub fn call_change(&self) {
        if let Some(on_change) = self.on_change.as_ref() {
            let raw:Vec<RawTrace> = 
                self.list.lock_ref()
                    .iter()
                    .map(|trace| trace.to_raw())
                    .collect();

            on_change(raw);
        }
    }


}


