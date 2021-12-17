use std::rc::Rc;
use futures_signals::signal_vec::MutableVec;
use shared::domain::jig::JigResponse;

pub struct JigUI {
  pub jigs: MutableVec<JigResponse>,
}

impl JigUI {
  pub fn new() -> Rc<Self> {
    Rc::new(Self {
      jigs: MutableVec::new(),
    })
  }
}
