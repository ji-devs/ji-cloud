use super::super::state::ModuleEnding;
use futures_signals::signal::Mutable;
use std::rc::Rc;

pub struct Ending {
    pub kind: Option<ModuleEnding>,
    pub ending_finished: Mutable<bool>,
}

impl Ending {
    pub fn new(kind: Option<ModuleEnding>) -> Rc<Self> {
        Rc::new(Self {
            kind,
            ending_finished: Mutable::new(kind.is_none()),
        })
    }
}
