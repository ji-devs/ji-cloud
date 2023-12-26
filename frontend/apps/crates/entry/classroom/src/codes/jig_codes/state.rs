use std::rc::Rc;

use futures_signals::{signal::Mutable, signal_vec::MutableVec};
use shared::domain::jig::{codes::JigCodeResponse, JigId, JigResponse};

pub struct JigCodes {
    pub jig_id: JigId,
    pub jig: Mutable<Option<JigResponse>>,
    pub codes: MutableVec<JigCodeResponse>,
}

impl JigCodes {
    pub fn new(jig_id: JigId) -> Rc<Self> {
        Rc::new(Self {
            jig_id,
            jig: Default::default(),
            codes: Default::default(),
        })
    }
}
