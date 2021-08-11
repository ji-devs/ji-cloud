use awsm_web::loaders::helpers::AsyncLoader;
use futures_signals::signal::Mutable;

pub struct State {
    pub loader: AsyncLoader,
    pub error: Mutable<bool>,
}

impl State {
    pub fn new() -> Self {
        Self {
            loader: AsyncLoader::new(),
            error: Mutable::new(false),
        }
    }
}
