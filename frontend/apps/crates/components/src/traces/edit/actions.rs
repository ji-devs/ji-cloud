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
    draw::state::*,
    all::trace::state::*,
};
use crate::traces::utils::TraceExt;
use utils::{
    prelude::*, 
    drag::Drag,
    resize::get_resize_info
};


impl Edit {
    pub fn delete_index(&self, index: usize) {
        self.selected_index.set(None);
        self.list.lock_mut().remove(index);
        self.call_change();
        /*
        self.get_history().push_modify(|game_data| {
            game_data.pairs.remove(pair_index);
        });
        */
    }

    pub fn duplicate(&self, index: usize) {
        {
            let mut list = self.list.lock_mut();
            if let Some(trace) = list.get(index) {
                let mut trace = trace.to_raw();
                let mut translation = &mut trace.transform.translation.0;

                //TODO - make the nudging random
                translation[0] += 0.01;
                translation[1] -= 0.01;
                let resize_info = get_resize_info();
                let trace = Rc::new(AllTrace::new(trace, &resize_info));

                list.push_cloned(trace);
                self.select_index(list.len()-1);
            }
        }
        self.call_change();
    }

    pub fn select_index(&self, index:usize) {
        self.selected_index.set(Some(index));
    }


    pub fn deselect(&self) {
        self.selected_index.set(None);
    }

    pub fn start_new_trace(_self: Rc<Self>, replace_index: Option<usize>, init: Option<(i32, i32)>) {

        _self.selected_index.set(replace_index); // a bit ugly but consistently deselects/selects

        let init_trace = replace_index.and_then(|index| {
            _self.list.lock_ref()
                .get(index)
                .map(|trace| trace.to_raw())
        });

        let draw = Draw::new(init_trace, clone!(_self => move |raw| {
            //On finished
            if let Some(raw) = raw {

                let resize_info = get_resize_info();
                let mut list = _self.list.lock_mut();
                
                let trace = Rc::new(AllTrace::new(raw, &resize_info));

                match replace_index {
                    None => {
                        list.push_cloned(trace);
                        _self.select_index(list.len()-1);
                    },
                    Some(index) => {
                        list.set_cloned(index, trace);
                    }
                }

                _self.phase.set(Phase::All);
            }
        }));

        if let Some((x, y)) = init {
            draw.start_draw(x, y);
        }
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


