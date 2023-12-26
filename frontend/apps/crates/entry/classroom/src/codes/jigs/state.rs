use std::rc::Rc;

use futures_signals::signal_vec::MutableVec;
use shared::domain::jig::JigResponse;

pub struct Jigs {
    pub jigs: MutableVec<JigResponse>,
}

impl Jigs {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            jigs: Default::default(),
        })
    }
}
