use std::rc::Rc;

use futures_signals::signal::Mutable;

use crate::pro_dev::state::ProDevPlayer;

pub struct PlayerMain {
    pub player_state: Rc<ProDevPlayer>,
    pub pro_dev_liked: Mutable<Option<bool>>,
}

impl PlayerMain {
    pub fn new(player_state: &Rc<ProDevPlayer>) -> Rc<Self> {
        Rc::new(Self {
            player_state: Rc::clone(&player_state),
            pro_dev_liked: Default::default(),
        })
    }
}
