use std::{collections::HashMap, rc::Rc};

use components::qr_dialog::QrDialog;
use futures_signals::signal::Mutable;
use shared::domain::{
    jig::{
        codes::{JigCode, JigCodeResponse, JigCodeSessionResponse},
        JigId, JigResponse,
    },
    module::{ModuleResponse, StableModuleId},
};

pub struct CodeSessions {
    pub(super) code: JigCode,
    pub(super) code_response: Mutable<Option<JigCodeResponse>>,
    pub(super) jig_id: JigId,
    pub(super) jig: Mutable<Option<JigWithModules>>,
    pub(super) infos: Mutable<Vec<JigCodeSessionResponse>>,
    pub(super) preview_open: Mutable<bool>,
    pub(super) qr_dialog: Mutable<Option<Rc<QrDialog>>>,
}

impl CodeSessions {
    pub fn new(jig_id: JigId, code: JigCode) -> Rc<Self> {
        Rc::new(Self {
            code,
            code_response: Default::default(),
            jig_id,
            jig: Default::default(),
            infos: Default::default(),
            preview_open: Default::default(),
            qr_dialog: Default::default(),
        })
    }
}

#[derive(Debug, Clone)]
pub(super) struct JigWithModules {
    pub jig: JigResponse,
    pub modules: HashMap<StableModuleId, ModuleResponse>,
}
