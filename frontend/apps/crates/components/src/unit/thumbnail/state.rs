use std::rc::Rc;

use futures_signals::signal::Mutable;

use shared::domain::pro_dev::unit::ProDevUnitValue;

pub struct UnitThumbnail {
    pub unit_value: Mutable<Option<ProDevUnitValue>>,
}

impl UnitThumbnail {
    pub fn new(
        unit_value: ProDevUnitValue,
    ) -> Rc<Self> {
        Rc::new(UnitThumbnail {
            unit_value: Mutable::new(Some(unit_value)),
        })
    }
}