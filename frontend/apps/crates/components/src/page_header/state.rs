use awsm_web::loaders::helpers::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::user::UserProfile;
use strum_macros::EnumIter;
use utils::routes::{HomeRoute, JigRoute, Route};

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

#[derive(Clone, Debug, PartialEq, EnumIter)]
pub enum PageLinks {
    Home,
    Content,
    Create,
    Community,
    Classroom,
    About,
}

impl PageLinks {
    pub fn kind_str(&self) -> &'static str {
        match self {
            Self::Home => "home",
            Self::Content => "content",
            Self::Create => "create",
            Self::Community => "community",
            Self::Classroom => "classroom",
            Self::About => "about",
        }
    }
    pub fn route(&self) -> &Route {
        match self {
            Self::Home => &Route::Home(HomeRoute::Home),
            Self::Content => &Route::Home(HomeRoute::Home),
            Self::Create => &Route::Jig(JigRoute::Gallery),
            Self::Community => &Route::Home(HomeRoute::Home),
            Self::Classroom => &Route::Home(HomeRoute::Home),
            Self::About => &Route::Home(HomeRoute::Home),
        }
    }
}
