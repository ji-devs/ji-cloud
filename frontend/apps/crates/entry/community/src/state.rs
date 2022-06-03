use std::rc::Rc;

pub struct Community {

}

impl Community {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {

        })
    }
}
