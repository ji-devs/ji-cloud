use futures_signals::signal::Mutable;
use shared::domain::pro_dev::unit::ProDevUnitValue;
use std::rc::Rc;

pub struct UnitValueView {
    pub unit_value: Mutable<Option<ProDevUnitValue>>,
}

impl UnitValueView {
    pub fn new(unit_value: Option<ProDevUnitValue>) -> Rc<Self> {
        Rc::new(Self {
            unit_value: Mutable::new(unit_value),
        })
    }
}
