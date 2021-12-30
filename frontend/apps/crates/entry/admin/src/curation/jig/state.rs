use dominator_helpers::futures::AsyncLoader;
use shared::domain::jig::JigId;
use std::rc::Rc;

use crate::curation::Curation;

pub struct CurationJig {
    pub jig_id: JigId,
    pub loader: AsyncLoader,
    pub curation_state: Rc<Curation>,
}

impl CurationJig {
    pub fn new(curation_state: Rc<Curation>, jig_id: JigId) -> Rc<Self> {
        Rc::new(Self {
            jig_id,
            loader: AsyncLoader::new(),
            curation_state,
        })
    }
}
