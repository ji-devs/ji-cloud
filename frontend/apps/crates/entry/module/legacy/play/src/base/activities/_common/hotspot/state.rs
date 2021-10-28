use futures_signals::signal::Mutable;
use shared::domain::jig::module::body::legacy::activity::Hotspot as RawHotspot;
use std::rc::Rc;

pub struct Hotspot {
    pub raw: RawHotspot,
    pub tooltip_text: Mutable<Option<String>>
}

impl Hotspot {
    pub fn new(raw: RawHotspot) -> Rc<Self> {
        Rc::new(Self {
            raw,
            tooltip_text: Mutable::new(None)
        })
    }
}