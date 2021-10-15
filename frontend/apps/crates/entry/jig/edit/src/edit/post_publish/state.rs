use std::rc::Rc;

use components::share_jig::state::State as ShareJigState;
use dominator_helpers::futures::AsyncLoader;
use shared::domain::jig::JigId;

use super::super::state::State as JigEditState;

pub struct State {
    pub jig_id: JigId,
    pub loader: AsyncLoader,
    pub share_state: Rc<ShareJigState>,
    pub jig_edit_state: Rc<JigEditState>,
}

impl State {
    pub fn new(jig_id: JigId, jig_edit_state: Rc<JigEditState>) -> Self {
        Self {
            jig_id: jig_id.clone(),
            loader: AsyncLoader::new(),
            share_state: Rc::new(ShareJigState::new(jig_id)),
            jig_edit_state,
        }
    }
}
