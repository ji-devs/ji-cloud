use futures_signals::signal::Mutable;
use shared::domain::course::unit::CourseUnitValue;
use std::rc::Rc;

pub struct UnitValueView {
    pub unit_value: Mutable<Option<CourseUnitValue>>,
}

impl UnitValueView {
    pub fn new(unit_value: Option<CourseUnitValue>) -> Rc<Self> {
        Rc::new(Self {
            unit_value: Mutable::new(unit_value),
        })
    }
}
