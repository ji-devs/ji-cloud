use std::rc::Rc;

use awsm_web::loaders::helpers::AsyncLoader;
use shared::domain::additional_resource::AdditionalResource;

use super::super::state::PrePublish as PublishState;

pub struct AdditionalResourceComponent {
    pub(super) loader: AsyncLoader,
    pub(super) publish_state: Rc<PublishState>,
    pub(super) additional_resource: AdditionalResource,
}

impl AdditionalResourceComponent {
    pub fn new(
        additional_resource: AdditionalResource,
        publish_state: Rc<PublishState>,
    ) -> Rc<Self> {
        Rc::new(Self {
            loader: AsyncLoader::new(),
            publish_state,
            additional_resource,
        })
    }
}
