use std::rc::Rc;

use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::pro_dev::unit::{ProDevUnitId, ProDevUnitValue};

use crate::edit::AssetEditState;

#[derive(Clone)]
pub struct UnitEditor {
    // Not having an ID means that's this is a new unit
    pub unit_id: Option<ProDevUnitId>,
    pub asset_edit_state: Rc<AssetEditState>,
    pub display_name: Mutable<String>,
    pub description: Mutable<String>,
    pub value: Mutable<Option<ProDevUnitValue>>,
    pub value_type: Mutable<Option<UnitValueType>>,
    pub loader: AsyncLoader,
}

impl UnitEditor {
    pub fn new(unit_id: Option<ProDevUnitId>, asset_edit_state: &Rc<AssetEditState>) -> Rc<Self> {
        Rc::new(Self {
            unit_id,
            asset_edit_state: Rc::clone(&asset_edit_state),
            display_name: Mutable::new("".to_string()),
            description: Mutable::new("".to_string()),
            value: Mutable::new(None),
            value_type: Mutable::new(Some(UnitValueType::Link)),
            loader: AsyncLoader::new(),
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub enum UnitValueType {
    /// Link for Unit
    Link,
    /// Upload file for Unit
    File,
    /// Link for Youtube video
    Youtube,
}
