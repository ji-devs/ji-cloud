use std::{cell::RefCell, rc::Rc};
use crate::base::state::Base;
use awsm_web::audio::AudioHandle;
use shared::domain::jig::module::body::legacy::activity::{SaySomething as RawSaySomething};
use dominator::clone;

pub struct SaySomething {
    pub base: Rc<Base>,
    pub raw: RawSaySomething,
    pub audio: RefCell<Option<AudioHandle>>
}

impl SaySomething {
    pub fn new(base: Rc<Base>, raw: RawSaySomething) -> Rc<Self> {
        let _self = Rc::new(Self{
            base,
            raw,
            audio: RefCell::new(None)
        });

        _self.base.set_bg_listener(clone!(_self => move || {
            _self.clone().on_bg_click();
        }));

        _self.base.insert_start_listener(clone!(_self => move || {
            _self.clone().on_start();
        }));

        _self
    }
}