use std::{cell::RefCell, rc::Rc};

use futures_signals::signal::Mutable;
use shared::domain::meta::ResourceType;
use url::Url;
use web_sys::File;

use super::super::state::AddAdditionalResource as AddAdditionalResourceState;

pub struct AddFile {
    pub file: Mutable<Option<File>>,
    pub resource_type: Mutable<Option<ResourceType>>,
    pub add_resources_state: Rc<AddAdditionalResourceState>,
}

impl AddFile {
    pub fn new(add_resources_state: Rc<AddAdditionalResourceState>) -> Rc<Self> {
        Rc::new(Self {
            file: Mutable::new(None),
            resource_type: Mutable::new(None),
            add_resources_state,
        })
    }
}
