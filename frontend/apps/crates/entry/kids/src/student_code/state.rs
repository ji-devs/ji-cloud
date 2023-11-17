use std::rc::Rc;

use awsm_web::loaders::helpers::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::jig::{JigId, JigPlayerSettings};

pub struct StudentCode {
    pub loader: AsyncLoader,
    pub error: Mutable<bool>,
    pub play_jig: Mutable<Option<(JigId, JigPlayerSettings)>>,
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
