use std::rc::Rc;

use futures_signals::signal::{Mutable, Signal};
use shared::domain::user::UserProfile;
use utils::{prelude::get_user, routes::Route};

pub struct Community {
    pub q: Mutable<String>,
    pub user: Mutable<Option<UserProfile>>,
}

impl Community {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            q: Mutable::default(),
            user: Mutable::new(get_user().cloned()),
        })
    }

    pub fn route_signal() -> impl Signal<Item = Route> {
        dominator::routing::url().signal_ref(|url| Route::from_url(url))
    }
}
