use crate::index::state::LocalState as IndexState;
use super::raw;
use std::rc::Rc;

pub struct State {
    pub index: Rc<IndexState>,
}


impl State {
    pub fn new(index: Rc<IndexState>, raw_data:raw::GameData) -> Self {
        Self {
            index
        }
    }
}
