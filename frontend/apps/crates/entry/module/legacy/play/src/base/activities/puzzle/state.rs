use crate::base::state::Base;
use std::rc::Rc;

use dominator::clone;
use shared::domain::jig::module::body::legacy::activity::Puzzle as RawPuzzle;

pub struct Puzzle {
    pub base: Rc<Base>,
    pub raw: RawPuzzle,
}

impl Puzzle {
    pub fn new(base: Rc<Base>, raw: RawPuzzle) -> Rc<Self> {
        let _self = Rc::new(Self { base, raw });

        _self.base.insert_start_listener(clone!(_self => move || {
            _self.clone().on_start();
        }));

        _self
    }
}
