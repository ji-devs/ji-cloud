use std::rc::Rc;

use futures_signals::signal::Mutable;
use shared::domain::meta::ResourceType;

use web_sys::File;

use super::super::state::UnitEditor;

pub struct AddFile {
    pub file: Mutable<Option<File>>,
    pub resource_type: Mutable<Option<ResourceType>>,
    pub unit_editor_state: Rc<UnitEditor>,
}

impl AddFile {
    pub fn new(unit_editor_state: Rc<UnitEditor>) -> Rc<Self> {
        Rc::new(Self {
            file: Mutable::new(None),
            resource_type: Mutable::new(None),
            unit_editor_state,
        })
    }
}
