use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal_vec::MutableVec;
use shared::domain::image::tag::ImageTagResponse;
use std::rc::Rc;

pub struct ImageTags {
    pub loader: AsyncLoader,
    pub list: MutableVec<Rc<ImageTagResponse>>,
}

impl ImageTags {
    pub fn new() -> Rc<Self> {
        let _self = Rc::new(Self {
            loader: AsyncLoader::new(),
            list: MutableVec::new(),
        });

        Self::load_init(_self.clone());

        _self
    }
}
