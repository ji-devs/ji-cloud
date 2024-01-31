use std::rc::Rc;

use futures_signals::signal::Mutable;
use shared::domain::{asset::AssetId, course::unit::CourseUnitId, module::ModuleId};
use utils::asset::{
    AssetPlayerOptions, CoursePlayerOptions, JigPlayerOptions, PlaylistPlayerOptions,
};

use super::PreviewPopupCallbacks;

pub struct PlayerPopup {
    pub asset_id: AssetId,
    pub module_id: Option<ModuleId>,
    pub unit_id: Option<CourseUnitId>,
    pub player_options: AssetPlayerOptions,
    pub open: Mutable<bool>,
    pub callbacks: PreviewPopupCallbacks,
    pub close_button_shown: Mutable<bool>,
}

impl PlayerPopup {
    pub fn new(
        asset_id: AssetId,
        module_id: Option<ModuleId>,
        unit_id: Option<CourseUnitId>,
        player_options: AssetPlayerOptions,
        callbacks: PreviewPopupCallbacks,
    ) -> Rc<Self> {
        Rc::new(Self {
            asset_id,
            module_id,
            player_options,
            unit_id,
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
            AssetId::PlaylistId(_) => PlaylistPlayerOptions::default().into(),
            AssetId::ResourceId(_) => unreachable!(),
            AssetId::CourseId(_) => CoursePlayerOptions::default().into(),
        };
        Rc::new(Self {
            asset_id,
            module_id: None,
            player_options,
            unit_id: None,
            open: Mutable::new(true),
            callbacks,
            close_button_shown: Mutable::new(true),
        })
    }

    pub fn new_default_player_options_with_jig_quota(
        asset_id: AssetId,
        callbacks: PreviewPopupCallbacks,
    ) -> Rc<Self> {
        let player_options = match asset_id {
            AssetId::JigId(_) => JigPlayerOptions {
                quota: true,
                ..Default::default()
            }
            .into(),
            AssetId::PlaylistId(_) => PlaylistPlayerOptions::default().into(),
            AssetId::ResourceId(_) => unreachable!(),
            AssetId::CourseId(_) => CoursePlayerOptions::default().into(),
        };
        Rc::new(Self {
            asset_id,
            module_id: None,
            player_options,
            unit_id: None,
            open: Mutable::new(true),
            callbacks,
            close_button_shown: Mutable::new(true),
        })
    }
}
