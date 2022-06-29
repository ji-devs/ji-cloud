use std::rc::Rc;

use futures_signals::signal::Mutable;
use shared::domain::asset::AssetId;
use utils::jig::JigPlayerOptions;

use super::PreviewPopupCallbacks;

pub struct PlayerPopup {
    pub asset_id: AssetId,
    pub player_options: JigPlayerOptions,
    pub open: Mutable<bool>,
    pub callbacks: PreviewPopupCallbacks,
}

impl PlayerPopup {
    pub fn new(
        asset_id: AssetId,
        player_options: JigPlayerOptions,
        callbacks: PreviewPopupCallbacks,
    ) -> Rc<Self> {
        Rc::new(Self {
            asset_id,
            player_options,
            open: Mutable::new(true),
            callbacks,
        })
    }
}
