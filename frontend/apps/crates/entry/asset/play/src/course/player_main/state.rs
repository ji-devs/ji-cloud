use std::rc::Rc;

use futures_signals::signal::Mutable;
use shared::domain::course::unit::CourseUnit;

use crate::course::state::CoursePlayer;

pub struct PlayerMain {
    pub player_state: Rc<CoursePlayer>,
    pub course_liked: Mutable<Option<bool>>,
    pub render_popup: Mutable<bool>,
    pub read_more: Mutable<Option<CourseUnit>>,
}

impl PlayerMain {
    pub fn new(player_state: &Rc<CoursePlayer>) -> Rc<Self> {
        Rc::new(Self {
            player_state: Rc::clone(&player_state),
            course_liked: Default::default(),
            render_popup: Mutable::new(false),
            read_more: Mutable::new(None),
        })
    }
}
