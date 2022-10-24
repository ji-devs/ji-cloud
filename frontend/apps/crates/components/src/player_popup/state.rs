use std::rc::Rc;

use futures_signals::signal::Mutable;
use shared::domain::{asset::AssetId, module::ModuleId};
use utils::asset::{AssetPlayerOptions, CoursePlayerOptions, JigPlayerOptions};

use super::PreviewPopupCallbacks;

pub struct PlayerPopup {
    pub asset_id: AssetId,
    pub module_id: Option<ModuleId>,
    pub player_options: AssetPlayerOptions,
    pub open: Mutable<bool>,
    pub callbacks: PreviewPopupCallbacks,
    pub close_button_shown: Mutable<bool>,
}

impl PlayerPopup {
    pub fn new(
        asset_id: AssetId,
        module_id: Option<ModuleId>,
        player_options: AssetPlayerOptions,
        callbacks: PreviewPopupCallbacks,
    ) -> Rc<Self> {
        Rc::new(Self {
            asset_id,
            module_id,
            player_options,
            open: Mutable::new(true),
            callbacks,
            close_button_shown: Mutable::new(true),
        })
    }

    pub fn new_default_player_options(
        asset_id: AssetId,
        callbacks: PreviewPopupCallbacks,
    ) -> Rc<Self> {
        let player_options = match asset_id {
            AssetId::JigId(_) => JigPlayerOptions::default().into(),
            AssetId::CourseId(_) => CoursePlayerOptions::default().into(),
            AssetId::ResourceId(_) => unreachable!(),
        };
        Rc::new(Self {
            asset_id,
            module_id: None,
            player_options,
            open: Mutable::new(true),
            callbacks,
            close_button_shown: Mutable::new(true),
        })
    }
}
