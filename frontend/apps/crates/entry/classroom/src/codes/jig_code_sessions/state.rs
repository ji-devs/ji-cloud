use std::{collections::HashMap, rc::Rc};

use futures_signals::signal::Mutable;
use shared::domain::{
    jig::{
        codes::{JigCode, JigCodeSessionResponse},
        JigId, JigResponse,
    },
    module::{ModuleResponse, StableModuleId},
};

pub struct CodeSessions {
    pub(super) code: JigCode,
    pub(super) jig_id: JigId,
    pub(super) jig: Mutable<Option<JigWithModules>>,
    pub(super) infos: Mutable<Vec<JigCodeSessionResponse>>,
    pub(super) preview_open: Mutable<bool>,
}

impl CodeSessions {
    pub fn new(jig_id: JigId, code: JigCode) -> Rc<Self> {
        Rc::new(Self {
            code,
            jig_id,
            jig: Default::default(),
            infos: Default::default(),
            preview_open: Default::default(),
        })
    }
}

#[derive(Debug, Clone)]
pub(super) struct JigWithModules {
    pub jig: JigResponse,
    pub modules: HashMap<StableModuleId, ModuleResponse>,
}
