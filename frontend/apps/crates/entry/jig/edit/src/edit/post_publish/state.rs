use std::rc::Rc;

use components::share_jig::state::State as ShareJigState;
use dominator_helpers::futures::AsyncLoader;
use shared::domain::jig::JigId;

pub struct State {
    pub jig_id: JigId,
    pub loader: AsyncLoader,
    pub share_state: Rc<ShareJigState>,
}

impl State {
    pub fn new(jig_id: JigId) -> Self {
        Self {
            jig_id: jig_id.clone(),
            loader: AsyncLoader::new(),
            share_state: Rc::new(ShareJigState::new(jig_id)),
        }
    }
}
