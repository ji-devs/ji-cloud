use std::rc::Rc;

use futures_signals::signal_vec::MutableVec;
use shared::domain::jig::codes::JigCodeResponse;

pub struct Codes {
    pub codes: MutableVec<JigCodeResponse>,
}

impl Codes {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            codes: Default::default(),
        })
    }
}
