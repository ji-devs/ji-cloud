use futures_signals::signal::Mutable;
use shared::domain::user::UserProfile;
use dominator_helpers::futures::AsyncLoader;

pub struct State {
    pub status: Mutable<Option<Result<UserProfile, ()>>>,
    pub loader: AsyncLoader
}

impl State {
    pub fn new() -> Self {
        Self { 
            status: Mutable::new(None),
            loader: AsyncLoader::new()
        }
    }
}
