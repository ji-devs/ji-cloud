use awsm_web::loaders::helpers::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::user::UserProfile;

pub struct State {
    pub logged_in: Mutable<LoggedInState>,
    pub loader: AsyncLoader,
}

impl State {
    pub fn new() -> Self {
        Self {
            logged_in: Mutable::new(LoggedInState::Loading),
            loader: AsyncLoader::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum LoggedInState {
    LoggedIn(UserProfile),
    LoggedOut,
    Loading,
}
