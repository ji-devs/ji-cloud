use std::rc::Rc;

use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::{Mutable, Signal};
use shared::domain::user::{UserId, UserProfile};
use utils::{prelude::get_user_cloned, routes::Route};

pub struct Community {
    pub q: Mutable<String>,
    pub user: Mutable<Option<UserProfile>>,
    pub followers: Mutable<Option<Vec<UserId>>>,
    pub followings: Mutable<Option<Vec<UserId>>>,
    pub loader: AsyncLoader,
}

impl Community {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            q: Mutable::default(),
            user: Mutable::new(get_user_cloned()),
            followers: Default::default(),
            followings: Default::default(),
            loader: AsyncLoader::new(),
        })
    }

    pub fn route_signal() -> impl Signal<Item = Route> {
        dominator::routing::url().signal_ref(|url| Route::from_url(url))
    }
}
