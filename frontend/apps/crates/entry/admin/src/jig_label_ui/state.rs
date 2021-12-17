use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal_vec::MutableVec;
use shared::domain::jig::JigResponse;
use std::rc::Rc;

pub struct JigUI {
  pub jigs: MutableVec<JigResponse>,
  pub loader: AsyncLoader,
}

impl JigUI {
  pub fn new() -> Rc<Self> {
    Rc::new(Self {
      jigs: MutableVec::new(),
      loader: AsyncLoader::new(),
    })
  }
}
