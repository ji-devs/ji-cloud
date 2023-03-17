use std::rc::Rc;

use futures_signals::signal::Mutable;
use url::Url;

use super::super::state::AddUnitValue as AddUnitValueState;

pub struct AddLink {
    pub url: Mutable<Option<Url>>,
    pub add_unit_value_state: Rc<AddUnitValueState>,
}

impl AddLink {
    pub fn new(add_unit_value_state: Rc<AddUnitValueState>) -> Rc<Self> {
        Rc::new(Self {
            url: Mutable::new(None),
            add_unit_value_state,
        })
    }
}
