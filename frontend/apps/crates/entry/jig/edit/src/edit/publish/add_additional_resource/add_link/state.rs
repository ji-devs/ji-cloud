use std::{rc::Rc};

use futures_signals::signal::Mutable;
use shared::domain::meta::ResourceType;
use url::Url;

use super::super::state::AddAdditionalResource as AddAdditionalResourceState;

pub struct AddLink {
    pub url: Mutable<Option<Url>>,
    pub resource_type: Mutable<Option<ResourceType>>,
    pub add_resources_state: Rc<AddAdditionalResourceState>,
}

impl AddLink {
    pub fn new(add_resources_state: Rc<AddAdditionalResourceState>) -> Rc<Self> {
        Rc::new(Self {
            url: Mutable::new(None),
            resource_type: Mutable::new(None),
            add_resources_state,
        })
    }
}
