use std::rc::Rc;

use shared::domain::jig::codes::JigCode;

pub struct CodeSessions {
    pub code: JigCode,
}

impl CodeSessions {
    pub fn new(code: JigCode) -> Rc<Self> {
        Rc::new(Self { code })
    }
}
