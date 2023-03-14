use std::rc::Rc;

use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::Mutable;

use crate::edit::pro_dev::unit_editor::UnitValueType;

use super::super::state::UnitEditor as UnitEditorState;

pub struct AddUnitValue {
    pub(super) loader: AsyncLoader,
    // pub(super) unit_type: UnitValueType,
    pub(super) unit_editor_state: Rc<UnitEditorState>,
}

impl AddUnitValue {
    pub fn new(unit_editor_state: Rc<UnitEditorState>) -> Rc<Self> {
        Rc::new(Self {
            // unit_type,
            loader: AsyncLoader::new(),
            unit_editor_state,
        })
    }
}

// #[derive(Clone, Copy, Debug)]
// pub(super) enum ActiveValueSelector {
//     File,
//     Link,
//     Youtube,
// }
