use futures_signals::signal::Mutable;
use std::rc::Rc;
use web_sys::File;

use super::super::state::AddUnitValue as AddUnitValueState;

pub struct AddFile {
    pub file: Mutable<Option<File>>,
    pub filename: Mutable<String>,
    pub add_unit_value_state: Rc<AddUnitValueState>,
}

impl AddFile {
    pub fn new(add_unit_value_state: Rc<AddUnitValueState>) -> Rc<Self> {
        Rc::new(Self {
            file: Mutable::new(None),
            filename: Mutable::new("".to_string()),
            add_unit_value_state,
        })
    }
}
