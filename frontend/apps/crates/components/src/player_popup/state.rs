use futures_signals::signal::Mutable;
use shared::domain::jig::{JigId, JigPlayerSettings};

use super::PreviewPopupCallbacks;


pub struct PlayerPopup {
    pub jig_id: JigId,
    pub player_settings: JigPlayerSettings,
    pub open: Mutable<bool>,
    pub callbacks: PreviewPopupCallbacks,
}

impl PlayerPopup {
    pub fn new(jig_id: JigId, player_settings: JigPlayerSettings, callbacks: PreviewPopupCallbacks) -> Self {
        Self {
            jig_id,
            player_settings,
            open: Mutable::new(true),
            callbacks,
        }
    }
}
