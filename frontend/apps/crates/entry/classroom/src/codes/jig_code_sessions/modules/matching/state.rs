use std::rc::Rc;

use shared::domain::{jig::codes::JigPlaySessionMatching, module::body::matching};

pub struct Matching {
    pub module: matching::Content,
    pub session: JigPlaySessionMatching,
}

impl Matching {
    pub fn new(module: matching::Content, session: JigPlaySessionMatching) -> Rc<Self> {
        Rc::new(Self { module, session })
    }
}
