use shared::domain::jig::module::body::legacy::activity::Hotspot as RawHotspot;
use std::rc::Rc;

pub struct Hotspot {
    pub raw: RawHotspot 
}

impl Hotspot {
    pub fn new(raw: RawHotspot) -> Rc<Self> {
        Rc::new(Self {
            raw 
        })
    }
}