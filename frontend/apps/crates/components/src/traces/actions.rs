use super::{
    state::*,
    trace::edit::state::Edit
};
use std::rc::Rc;
use shared::domain::jig::module::body::{Trace, Transform};
use utils::{
    prelude::*, 
    drag::Drag,
    resize::get_resize_info
};

impl Traces {
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

    pub fn start_new_trace(&self) {
        let edit = Edit::new(None, self.on_change_cb.borrow().as_ref().unwrap_ji().clone());
        self.phase.set(Phase::Edit(Rc::new(edit)));
    }
    // Internal - saving/history is done on the module level
    pub fn call_change(&self) {
        if let Some(on_change) = self.on_change.as_ref() {
            let raw:Vec<Trace> = 
                self.list.lock_ref()
                    .iter()
                    .map(|trace| trace.to_raw())
                    .collect();

            on_change(raw);
        }
    }


}


