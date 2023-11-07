use std::rc::Rc;

use shared::domain::jig::player::JigPlayerSessionIndex;

pub struct CodeSessions {
    pub code: JigPlayerSessionIndex,
}

impl CodeSessions {
    pub fn new(code: JigPlayerSessionIndex) -> Rc<Self> {
        Rc::new(Self { code })
    }
}
