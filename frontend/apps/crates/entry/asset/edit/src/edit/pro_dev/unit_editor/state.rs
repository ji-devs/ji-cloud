use std::rc::Rc;

use shared::domain::pro_dev::unit::ProDevUnitId;

use crate::edit::AssetEditState;

pub struct UnitEditor {
    // Not having an ID means that's this is a new unit
    pub unit_id: Option<ProDevUnitId>,
    pub asset_edit_state: Rc<AssetEditState>,
}

impl UnitEditor {
    pub fn new(unit_id: Option<ProDevUnitId>, asset_edit_state: &Rc<AssetEditState>) -> Rc<Self> {
        Rc::new(Self {
            unit_id,
            asset_edit_state: Rc::clone(&asset_edit_state),
        })
    }
}
