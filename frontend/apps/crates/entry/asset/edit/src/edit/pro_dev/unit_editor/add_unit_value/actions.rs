use std::rc::Rc;

use shared::domain::additional_resource::CreateAssetResourcePath;
use shared::domain::additional_resource::ResourceContent;
use shared::domain::meta::ResourceTypeId;

use utils::prelude::ApiEndpointExt;

use super::state::AddUnitValue;

impl AddUnitValue {
    pub(super) async fn save_additional_resource(
        self: &Rc<Self>,
        resource_content: ResourceContent,
        display_name: String,
        resource_type_id: ResourceTypeId,
    ) {
        let state = Rc::clone(self);
    }
}
