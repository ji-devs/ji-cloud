use crate::{home::Home, plan::Plan, pricing::Pricing};

use components::overlay::container::OverlayContainer;
use dominator::{html, Dom};
use futures_signals::signal::Signal;
use utils::routes::{HomeRoute, Route};

use crate::help_center;
pub struct Router {}

impl Router {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render() -> Dom {
        html!("main", {
            .style("display", "grid")
            .child_signal(Self::dom_signal())
            .child(OverlayContainer::new().render(None))
        })
    }

    fn dom_signal() -> impl Signal<Item = Option<Dom>> {
        dominator::routing::url().signal_ref(|url| {
            let route = Route::from_url(url);
            match route {
                Route::Home(route) => match route {
                    HomeRoute::Home => Some(Home::new().render(false)),
                    HomeRoute::Search(search_query) => {
                        let search_query = search_query.map(|x| *x);
                        Some(Home::new_search(search_query).render(true))
                    }
                    HomeRoute::Help => Some(help_center::render_help_center()),
                    HomeRoute::Pricing(route) => Some(Pricing::new(route).render()),
                    HomeRoute::Plan(route) => Some(Plan::new(route).render()),
                },
                _ => None,
            }
        })
    }
}
