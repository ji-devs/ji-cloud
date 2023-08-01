use std::rc::Rc;

use crate::subscribe1::Subscribe1;

pub struct CheckPopup {
    pub subscribe_1_state: Rc<Subscribe1>,
}
impl CheckPopup {
    pub fn new(subscribe_1_state: &Rc<Subscribe1>) -> Rc<Self> {
        Rc::new(Self {
            subscribe_1_state: Rc::clone(&subscribe_1_state),
        })
    }
}
