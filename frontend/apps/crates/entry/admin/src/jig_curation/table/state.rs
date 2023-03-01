use dominator_helpers::futures::AsyncLoader;
use std::rc::Rc;

use crate::jig_curation::JigCuration;

pub struct JigTable {
    pub loader: AsyncLoader,
    pub curation_state: Rc<JigCuration>,
}

impl JigTable {
    pub fn new(curation_state: Rc<JigCuration>) -> Rc<Self> {
        Rc::new(Self {
            loader: AsyncLoader::new(),
            curation_state,
        })
    }
}
