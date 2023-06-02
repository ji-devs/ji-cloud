use std::rc::Rc;

use super::PlayerPopup;
use dominator::clone;
use futures_signals::signal::{Signal, SignalExt};
use shared::domain::course::CourseResponse;

impl PlayerPopup {
    pub fn navigate_previous_signal(&self) -> impl Signal<Item = bool> {
        self.player_state.active_unit.signal().map(
            move |active_unit| {
                if active_unit > Some(0) {
                    false
                } else {
                    true
                }
            },
        )
    }

    pub fn navigate_forward_signal(&self, course: &Rc<CourseResponse>) -> impl Signal<Item = bool> {
        self.player_state
            .active_unit
            .signal()
            .map(clone!(course => move |active_unit| {
                let last_index = course.course_data.units.len();
                if active_unit < Some(last_index - 1)  {
                    false
                } else {
                    true
                }
            }))
    }
}
