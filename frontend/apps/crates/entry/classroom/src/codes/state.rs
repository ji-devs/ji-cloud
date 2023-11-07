use std::rc::Rc;

pub struct Codes {}

impl Codes {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {})
    }
}
