use std::rc::Rc;

use futures_signals::signal::Mutable;
use shared::domain::module::ModuleKind;
use utils::drag::Drag;

use crate::edit::AssetEditState;

pub struct ModuleSelection {
    pub asset_edit_state: Rc<AssetEditState>,
    pub drag: Mutable<Option<Rc<Drag<ModuleKind>>>>,
}

impl ModuleSelection {
    pub fn new(asset_edit_state: &Rc<AssetEditState>) -> Rc<Self> {
        Rc::new(Self {
            asset_edit_state: Rc::clone(asset_edit_state),
            drag: Default::default(),
        })
    }
}
