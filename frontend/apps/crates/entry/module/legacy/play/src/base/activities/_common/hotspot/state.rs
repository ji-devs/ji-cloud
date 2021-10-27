use shared::domain::jig::module::body::_groups::design::{TraceShape};
use std::rc::Rc;

pub struct Hotspot {
    pub shape: TraceShape 
}

impl Hotspot {
    pub fn new(shape: TraceShape) -> Rc<Self> {
        Rc::new(Self {
            shape 
        })
    }
}