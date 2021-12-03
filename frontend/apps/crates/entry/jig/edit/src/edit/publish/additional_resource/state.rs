use std::rc::Rc;

use awsm_web::loaders::helpers::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::jig::additional_resource::{AdditionalResource, AdditionalResourceId};

use super::super::state::Publish as PublishState;

pub struct AdditionalResourceComponent {
    pub(super) loader: AsyncLoader,
    pub(super) id: AdditionalResourceId,
    pub(super) publish_state: Rc<PublishState>,
    pub(super) additional_resource: Mutable<Option<AdditionalResource>>,
}

impl AdditionalResourceComponent {
    pub fn new(id: AdditionalResourceId, publish_state: Rc<PublishState>) -> Rc<Self> {
        Rc::new(Self {
            loader: AsyncLoader::new(),
            id,
            publish_state,
            additional_resource: Mutable::new(None),
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum ResourceMode {
    File,
    Link,
}
