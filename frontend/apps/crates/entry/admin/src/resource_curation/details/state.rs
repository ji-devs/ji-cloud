use dominator_helpers::futures::AsyncLoader;
use shared::domain::resource::ResourceId;
use std::rc::Rc;

use crate::resource_curation::ResourceCuration;
use utils::editable_asset::EditableResource;

pub struct ResourceDetails {
    pub resource_id: ResourceId,
    pub resource: Rc<EditableResource>,
    pub loader: AsyncLoader,
    pub curation_state: Rc<ResourceCuration>,
}

impl ResourceDetails {
    pub fn new(
        curation_state: Rc<ResourceCuration>,
        resource_id: ResourceId,
        resource: Rc<EditableResource>,
    ) -> Rc<Self> {
        Rc::new(Self {
            resource_id,
            resource,
            loader: AsyncLoader::new(),
            curation_state,
        })
    }
}
