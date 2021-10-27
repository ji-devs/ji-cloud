use awsm_web::loaders::helpers::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::user::UserProfile;
use strum_macros::EnumIter;
use utils::routes::{HomeRoute, JigRoute, Route};

const TARGET_SELF: &str = "_self";
const TARGET_BLANK: &str = "_blank";

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
    pub fn route(&self) -> String {
        match self {
            Self::Home => Route::Home(HomeRoute::Home).to_string(),
            Self::Content => Route::Home(HomeRoute::Search(None)).to_string(),
            Self::Create => Route::Jig(JigRoute::Gallery).to_string(),
            Self::Community => "javascript:alert(\"Coming soon\")".to_string(),
            Self::Classroom => "javascript:alert(\"Coming soon\")".to_string(),
            Self::About => "https://www.jewishinteractive.org/ji-about-us".to_string(),
        }
    }
    pub fn target(&self) -> &'static str {
        match self {
            Self::Home => TARGET_SELF,
            Self::Content => TARGET_SELF,
            Self::Create => TARGET_SELF,
            Self::Community => TARGET_SELF,
            Self::Classroom => TARGET_SELF,
            Self::About => TARGET_BLANK,
        }
    }
}
