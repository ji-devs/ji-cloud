use std::rc::Rc;

use futures_signals::signal::Mutable;
use shared::domain::{meta::ResourceType, pro_dev::unit::ProDevUnitValue};

use web_sys::File;

use super::super::state::AddUnitValue as AddUnitValueState;

pub struct AddFile {
    pub file: Mutable<Option<File>>,
    pub unit_type: Mutable<Option<ProDevUnitValue>>,
    pub add_unit_value_state: Rc<AddUnitValueState>,
}

impl AddFile {
    pub fn new(add_unit_value_state: Rc<AddUnitValueState>) -> Rc<Self> {
        Rc::new(Self {
            file: Mutable::new(None),
            unit_type: Mutable::new(None),
            add_unit_value_state,
        })
    }
}
