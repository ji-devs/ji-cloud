use super::super::state::Sidebar;
use std::rc::Rc;

pub struct Step1 {
    pub sidebar: Rc<Sidebar>,
}

impl Step1 {
    pub fn new(sidebar: Rc<Sidebar>) -> Rc<Self> {
        Rc::new(Self { sidebar })
    }
}
