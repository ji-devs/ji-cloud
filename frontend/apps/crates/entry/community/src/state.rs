use std::rc::Rc;

use futures_signals::signal::{Mutable, Signal};
use shared::domain::user::UserProfile;
use utils::{prelude::get_user, routes::Route, unwrap::UnwrapJiExt};

pub struct Community {
    pub user: Mutable<UserProfile>,
}

impl Community {
    pub fn new() -> Rc<Self> {
        let user = get_user().expect_ji("user not ready").clone();
        Rc::new(Self {
            user: Mutable::new(user),
        })
    }

    pub fn route_signal() -> impl Signal<Item = Route> {
        dominator::routing::url().signal_ref(|url| Route::from_url(url))
    }
}
