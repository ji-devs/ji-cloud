use components::overlay::container::OverlayContainer;
use utils::routes::*;

use dominator::{html, Dom};
use futures_signals::signal::{Signal, SignalExt};

pub struct Router {}

impl Router {
    pub fn new() -> Self {
        Self {}
    }

    fn signal() -> impl Signal<Item = Route> {
        dominator::routing::url().signal_ref(|url| Route::from_url(url))
    }

    fn dom_signal() -> impl Signal<Item = Option<Dom>> {
        Self::signal().map(|route| match route {
            Route::Community(route) => match route {
                CommunityRoute::Landing => Some(html!("div", {
                    .text("community")
                })),
            },
            _ => None,
        })
    }

    pub fn render(&self) -> Dom {
        html!("main", {
            .child_signal(Self::dom_signal())
            .child(OverlayContainer::new().render(None))
        })
    }
}
