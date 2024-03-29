use std::rc::Rc;

use shared::domain::course::unit::CourseUnitValue;
use utils::unwrap::UnwrapJiExt;

use super::state::AddLink;

impl AddLink {
    pub fn save(self: &Rc<Self>) {
        let state = Rc::clone(self);

        let url = self.url.get_cloned().unwrap_ji();

        let value = CourseUnitValue::Link(url);

        state
            .add_unit_value_state
            .unit_editor_state
            .changed
            .set(true);

        self.add_unit_value_state.loader.load(async move {
            state
                .add_unit_value_state
                .unit_editor_state
                .value
                .set(value.into());
        });
    }
}
