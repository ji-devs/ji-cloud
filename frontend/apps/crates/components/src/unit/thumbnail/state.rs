use std::rc::Rc;

use futures_signals::signal::Mutable;

use shared::domain::course::unit::CourseUnitValue;

pub struct UnitThumbnail {
    pub unit_value: Mutable<Option<CourseUnitValue>>,
}

impl UnitThumbnail {
    pub fn new(unit_value: CourseUnitValue) -> Rc<Self> {
        Rc::new(UnitThumbnail {
            unit_value: Mutable::new(Some(unit_value)),
        })
    }
}
