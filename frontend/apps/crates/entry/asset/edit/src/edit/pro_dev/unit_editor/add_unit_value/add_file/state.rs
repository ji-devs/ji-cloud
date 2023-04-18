use std::rc::Rc;

use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::Mutable;

use shared::domain::pro_dev::unit::ProDevUnitValue;
use web_sys::File;

use super::super::state::AddUnitValue as AddUnitValueState;

pub struct AddFile {
    pub file: Mutable<Option<File>>,
    pub filename: Mutable<String>,
    pub value: Mutable<Option<ProDevUnitValue>>,
    pub add_unit_value_state: Rc<AddUnitValueState>,
    pub loader: AsyncLoader,
}

impl AddFile {
    pub fn new(add_unit_value_state: Rc<AddUnitValueState>) -> Rc<Self> {
        Rc::new(Self {
            file: Mutable::new(None),
            filename: Mutable::new("".to_string()),
            value: Mutable::new(None),
            add_unit_value_state,
            loader: AsyncLoader::new(),
        })
    }
}
