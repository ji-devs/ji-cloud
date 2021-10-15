use std::rc::Rc;

use super::super::state::*;

pub struct Hints {
    pub game: Rc<Game>,
}

impl Hints {
    pub fn new(game: Rc<Game>) -> Rc<Self> {
        Rc::new(Self { game })
    }
}
