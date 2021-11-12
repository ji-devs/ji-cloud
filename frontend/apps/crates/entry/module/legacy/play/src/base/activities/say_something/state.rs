use crate::base::state::Base;
use std::rc::Rc;

use dominator::clone;
use shared::domain::jig::module::body::legacy::activity::SaySomething as RawSaySomething;

pub struct SaySomething {
    pub base: Rc<Base>,
    pub raw: RawSaySomething,
}

impl SaySomething {
    pub fn new(base: Rc<Base>, raw: RawSaySomething) -> Rc<Self> {
        let _self = Rc::new(Self { base, raw });

        _self.base.set_bg_listener(clone!(_self => move || {
            _self.clone().on_bg_click();
        }));

        _self.base.insert_start_listener(clone!(_self => move || {
            _self.clone().on_start();
        }));

        _self
    }
}
