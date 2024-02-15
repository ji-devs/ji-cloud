use std::rc::Rc;

use awsm_web::loaders::helpers::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::jig::{JigId, JigPlayerSettings};

pub struct StudentCode {
    pub loader: AsyncLoader,
    pub error: Mutable<bool>,
    pub(super) play_jig: Mutable<Option<PlayJig>>,
}

impl StudentCode {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            loader: AsyncLoader::new(),
            error: Mutable::new(false),
            play_jig: Mutable::new(None),
        })
    }
}

#[derive(Clone)]
pub(super) struct PlayJig {
    pub id: JigId,
    pub settings: JigPlayerSettings,
    pub token: String,
    pub name: Mutable<Option<String>>,
}
