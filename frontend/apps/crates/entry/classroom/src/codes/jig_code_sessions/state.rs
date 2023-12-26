use std::{cell::RefCell, collections::HashMap, rc::Rc};

use futures_signals::{signal::Mutable, signal_vec::MutableVec};
use shared::domain::{
    jig::{
        codes::{JigCode, JigCodeSessionResponse},
        JigId, JigResponse,
    },
    module::{ModuleId, ModuleResponse},
};

pub struct CodeSessions {
    pub code: JigCode,
    pub jig_id: JigId,
    pub jig: Mutable<Option<JigResponse>>,
    pub modules: RefCell<HashMap<ModuleId, ModuleResponse>>,
    pub infos: MutableVec<JigCodeSessionResponse>,
}

impl CodeSessions {
    pub fn new(jig_id: JigId, code: JigCode) -> Rc<Self> {
        Rc::new(Self {
            code,
            jig_id,
            jig: Default::default(),
            modules: Default::default(),
            infos: Default::default(),
        })
    }
}
