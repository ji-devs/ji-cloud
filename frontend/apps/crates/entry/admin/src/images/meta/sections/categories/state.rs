use crate::images::meta::{
    sections::common::categories::MutableCategory,
    state::{MutableImage, State as MetaState},
};
use std::rc::Rc;

pub struct State {
    pub meta: Rc<MetaState>,
    pub image: Rc<MutableImage>,
    pub categories: Rc<Vec<Rc<MutableCategory>>>,
}

impl State {
    pub fn new(
        meta: Rc<MetaState>,
        image: Rc<MutableImage>,
        categories: Rc<Vec<Rc<MutableCategory>>>,
    ) -> Self {
        Self {
            meta,
            image,
            categories,
        }
    }
}
