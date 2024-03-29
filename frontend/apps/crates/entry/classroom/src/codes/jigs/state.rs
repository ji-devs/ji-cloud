use std::rc::Rc;

use futures_signals::signal::Mutable;
use shared::domain::jig::codes::JigWithCodes;

pub struct Jigs {
    pub jigs: Mutable<Option<Vec<JigWithCodes>>>,
}

impl Jigs {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            jigs: Default::default(),
        })
    }
}
