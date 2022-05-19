use std::rc::Rc;

use super::super::state::*;

pub struct Hints {
    pub game: Rc<Game>,
}

impl Hints {
    // TODO find-answer
    #[allow(dead_code)]
    pub fn new(game: Rc<Game>) -> Rc<Self> {
        Rc::new(Self { game })
    }
}
