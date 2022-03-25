use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::jig::JigId;
use std::rc::Rc;

use crate::curation::{Curation, EditableJig};

pub struct CurationJig {
    pub jig_id: JigId,
    pub jig: Rc<EditableJig>,
    pub loader: AsyncLoader,
    pub curation_state: Rc<Curation>,
    pub player_open: Mutable<bool>,
}

impl CurationJig {
    pub fn new(curation_state: Rc<Curation>, jig_id: JigId, jig: Rc<EditableJig>) -> Rc<Self> {
        Rc::new(Self {
            jig_id,
            jig: jig.into(),
            loader: AsyncLoader::new(),
            curation_state,
            player_open: Mutable::new(false),
        })
    }
}
