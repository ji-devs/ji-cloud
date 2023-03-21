use std::rc::Rc;

use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::Mutable;

use super::super::state::UnitEditor as UnitEditorState;

pub struct AddUnitValue {
    pub(super) loader: AsyncLoader,
    pub(super) unit_editor_state: Rc<UnitEditorState>,
}

impl AddUnitValue {
    pub fn new(unit_editor_state: Rc<UnitEditorState>) -> Rc<Self> {
        Rc::new(Self {
            loader: AsyncLoader::new(),
            unit_editor_state,
        })
    }
}
