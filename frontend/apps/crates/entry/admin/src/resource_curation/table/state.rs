use dominator_helpers::futures::AsyncLoader;
use std::rc::Rc;

use crate::resource_curation::ResourceCuration;

pub struct ResourceTable {
    pub loader: AsyncLoader,
    pub curation_state: Rc<ResourceCuration>,
}

impl ResourceTable {
    pub fn new(curation_state: Rc<ResourceCuration>) -> Rc<Self> {
        Rc::new(Self {
            loader: AsyncLoader::new(),
            curation_state,
        })
    }
}
