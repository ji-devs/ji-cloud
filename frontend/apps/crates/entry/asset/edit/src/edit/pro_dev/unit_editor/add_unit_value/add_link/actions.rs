use std::rc::Rc;

use shared::domain::additional_resource::ResourceContent;
use utils::unwrap::UnwrapJiExt;

use super::state::AddLink;

impl AddLink {
    pub fn save(self: &Rc<Self>) {
        let state = Rc::clone(self);

        let url = self.url.get_cloned().unwrap_ji();
        let display_name = url.to_string();
        let resource_content = ResourceContent::Link(url);

        // self.add_resources_state.loader.load(async move {
        //     state
        //         .add_resources_state
        //         .save_additional_resource(resource_content, display_name, resource_type_id)
        //         .await;
        // });
    }
}
