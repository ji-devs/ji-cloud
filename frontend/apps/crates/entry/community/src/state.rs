use std::rc::Rc;

use futures_signals::signal::Signal;
use utils::routes::Route;

pub struct Community {}

impl Community {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {})
    }

    pub fn route_signal() -> impl Signal<Item = Route> {
        dominator::routing::url().signal_ref(|url| Route::from_url(url))
    }
}
