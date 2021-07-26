use std::rc::Rc;
use dominator_helpers::futures::AsyncLoader;
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt},
    signal_vec::{MutableVec, SignalVec, SignalVecExt},
};
use shared::domain::{
    meta::TagId,
    image::tag::ImageTagResponse
};

pub struct ImageTags {
    pub loader: AsyncLoader,
    pub list: MutableVec<Rc<ImageTagResponse>>
}

impl ImageTags {
    pub fn new() -> Rc<Self> {
        let _self = Rc::new(Self {
            loader: AsyncLoader::new(),
            list: MutableVec::new()
        });

        Self::load_init(_self.clone());

        _self
    }
}


