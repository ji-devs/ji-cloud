use std::{rc::Rc, str::FromStr};

use futures_signals::{signal::Mutable, signal_vec::MutableVec};
use shared::domain::{
    jig::codes::{JigCode, JigCodeSessionResponse},
    module::{ModuleId, ModuleResponse},
};
use utils::unwrap::UnwrapJiExt;

pub struct CodeSessions {
    pub code: JigCode,
    pub module_id: ModuleId,
    pub module: Mutable<Option<ModuleResponse>>,
    pub infos: MutableVec<JigCodeSessionResponse>,
}

impl CodeSessions {
    pub fn new(code: JigCode) -> Rc<Self> {
        Rc::new(Self {
            code,
            module_id: ModuleId(
                uuid::Uuid::from_str("fbf0502c-7a59-11ee-b017-3388f4a32383").unwrap_ji(),
            ),
            module: Default::default(),
            infos: Default::default(),
        })
    }
}
