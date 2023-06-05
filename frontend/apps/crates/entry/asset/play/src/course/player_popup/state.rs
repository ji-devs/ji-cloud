use std::rc::Rc;

use futures_signals::signal::Mutable;

use crate::course::state::CoursePlayer;

pub struct PlayerPopup {
    pub player_state: Rc<CoursePlayer>,
    pub is_full_screen: Mutable<bool>,
    pub read_more: Mutable<bool>,
    pub render_popup: Mutable<bool>,
    pub description: Mutable<Option<String>>,
    pub name: Mutable<Option<String>>
}

impl PlayerPopup {
    pub fn new(player_state: &Rc<CoursePlayer>) -> Rc<Self> {
        Rc::new(Self {
            player_state: Rc::clone(&player_state),
            is_full_screen: Default::default(),
            read_more: Mutable::new(false),
            render_popup: Mutable::new(false),
            description: Mutable::new(None),
            name: Mutable::new(None)
        })
    }
}
