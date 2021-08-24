use awsm_web::loaders::helpers::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::jig::{JigId, JigPlayerSettings};

pub struct State {
    pub loader: AsyncLoader,
    pub error: Mutable<bool>,
    pub play_jig: Mutable<Option<(JigId, JigPlayerSettings)>>,
}

impl State {
    pub fn new() -> Self {
        Self {
            loader: AsyncLoader::new(),
            error: Mutable::new(false),
            play_jig: Mutable::new(None),
        }
    }
}
