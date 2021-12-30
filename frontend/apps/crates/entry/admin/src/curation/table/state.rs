use dominator_helpers::futures::AsyncLoader;
use std::rc::Rc;

use crate::curation::Curation;

pub struct CurationTable {
    pub loader: AsyncLoader,
    pub curation_state: Rc<Curation>,
}

impl CurationTable {
    pub fn new(curation_state: Rc<Curation>) -> Rc<Self> {
        Rc::new(Self {
            loader: AsyncLoader::new(),
            curation_state,
        })
    }
}
