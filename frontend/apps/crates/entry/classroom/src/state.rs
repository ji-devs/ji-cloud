use std::rc::Rc;

pub struct Classroom {}

impl Classroom {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {})
    }
}
