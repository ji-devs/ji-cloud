use std::rc::Rc;
use futures_signals::signal_vec::MutableVec;

pub struct JigData {
  pub jig_name: String,
  pub author: String,
  pub author_badge: String,
  pub date: String,
  pub language: String,
  pub curators: Vec<String>,
}

pub struct JigUI {
  jigs: MutableVec<JigData>,
}

impl JigUI {
  pub fn new() -> Rc<Self> {
    Rc::new(Self {
      jigs: MutableVec::new(),
    })
  }
}
