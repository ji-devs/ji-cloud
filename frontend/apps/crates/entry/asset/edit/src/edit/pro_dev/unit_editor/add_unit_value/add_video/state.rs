use std::rc::Rc;

use components::stickers::video::state::Video;
use futures_signals::signal::Mutable;

use super::super::state::AddUnitValue as AddUnitValueState;

pub struct AddVideo {
    pub video: Mutable<Option<Video>>,
    pub add_unit_value_state: Rc<AddUnitValueState>,
}

impl AddVideo {
    pub fn new(add_unit_value_state: Rc<AddUnitValueState>) -> Rc<Self> {
        Rc::new(Self {
            video: Mutable::new(None),
            add_unit_value_state,
        })
    }
}
