use super::home;
use components::overlay::container::OverlayContainer;
use dominator::{html, Dom};
use futures_signals::signal::Signal;
use std::rc::Rc;
use utils::routes::{HomeRoute, Route};

pub struct Router {}

impl Router {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render() -> Dom {
        html!("main", {
            .child_signal(Self::dom_signal())
            .child(OverlayContainer::new().render(None))
        })
    }

    fn dom_signal() -> impl Signal<Item = Option<Dom>> {
        dominator::routing::url().signal_ref(|url| {
            let route = Route::from_url(url);
            match route {
                Route::Home(route) => match route {
                    HomeRoute::Home => {
                        Some(home::dom::render(Rc::new(home::state::State::new()), false))
                    }
                    HomeRoute::Search(search_query) => {
                        let state =
                            Rc::new(home::state::State::new_search(search_query.map(|x| *x)));
                        Some(home::dom::render(state, true))
                    }
                },
                _ => None,
            }
        })
    }
}
