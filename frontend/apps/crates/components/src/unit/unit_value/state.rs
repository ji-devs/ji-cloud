use futures_signals::signal::Mutable;
use shared::domain::course::unit::CourseUnitValue;
use std::rc::Rc;

pub struct UnitValueView {
    pub unit_value: Mutable<Option<CourseUnitValue>>,
    pub is_student: bool,
}

impl UnitValueView {
    pub fn new(unit_value: Option<CourseUnitValue>) -> Rc<Self> {
        Self::new_with_student(unit_value, false)
    }

    pub fn new_with_student(unit_value: Option<CourseUnitValue>, is_student: bool) -> Rc<Self> {
        Rc::new(Self {
            unit_value: Mutable::new(unit_value),
            is_student,
        })
    }
}
