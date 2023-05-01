use std::rc::Rc;

use futures_signals::signal::Mutable;

use crate::pro_dev::state::ProDevPlayer;

pub struct PlayerPopup {
    pub player_state: Rc<ProDevPlayer>,
    pub is_full_screen: Mutable<bool>,
}

impl PlayerPopup {
    pub fn new(player_state: &Rc<ProDevPlayer>) -> Rc<Self> {
        Rc::new(Self {
            player_state: Rc::clone(&player_state),
            is_full_screen: Default::default(),
        })
    }
}
