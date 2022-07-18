use std::rc::Rc;

use dominator::class;
use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::{Mutable, Signal};
use once_cell::sync::Lazy;
use shared::domain::user::{UserId, UserProfile};
use utils::{prelude::get_user, routes::Route};

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
            user: Mutable::new(get_user().cloned()),
            followers: Default::default(),
            followings: Default::default(),
            loader: AsyncLoader::new(),
        })
    }

    pub fn route_signal() -> impl Signal<Item = Route> {
        dominator::routing::url().signal_ref(|url| Route::from_url(url))
    }
}

pub(super) static CIRCLE_LIST_GRID_COLUMNS: Lazy<String> = Lazy::new(|| {
    class! {
        // 108px 20% 100px 1fr 20%
        .style("grid-template-columns", "108px 200px 50px 400px 138px")
    }
});

pub(super) static MEMBER_LIST_GRID_COLUMNS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("grid-template-columns", "64px 200px 100px 100px 138px")
    }
});
