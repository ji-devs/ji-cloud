use std::rc::Rc;

use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::Mutable;

use super::super::state::Publish as PublishState;

pub struct AddAdditionalResource {
    pub(super) active_popup: Mutable<Option<ActivePopup>>,
    pub(super) loader: AsyncLoader,
    pub(super) publish_state: Rc<PublishState>,
}

impl AddAdditionalResource {
    pub fn new(publish_state: Rc<PublishState>) -> Rc<Self> {
        Rc::new(Self {
            active_popup: Mutable::new(None),
            loader: AsyncLoader::new(),
            publish_state,
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum ActivePopup {
    Main,
    File,
    Link,
}
