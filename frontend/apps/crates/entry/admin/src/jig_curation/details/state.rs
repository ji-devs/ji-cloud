use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::jig::JigId;
use std::rc::Rc;

use crate::jig_curation::{EditableJig, JigCuration};

pub struct JigDetails {
    pub jig_id: JigId,
    pub jig: Rc<EditableJig>,
    pub loader: AsyncLoader,
    pub curation_state: Rc<JigCuration>,
    pub player_open: Mutable<bool>,
}

impl JigDetails {
    pub fn new(curation_state: Rc<JigCuration>, jig_id: JigId, jig: Rc<EditableJig>) -> Rc<Self> {
        Rc::new(Self {
            jig_id,
            jig,
            loader: AsyncLoader::new(),
            curation_state,
            player_open: Mutable::new(false),
        })
    }
}
