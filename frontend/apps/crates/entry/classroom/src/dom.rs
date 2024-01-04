use std::rc::Rc;

use components::{
    overlay::container::OverlayContainer,
    page_footer,
    page_header::{PageHeader, PageHeaderConfig},
};
use utils::{component::Component, routes::*};

use dominator::{html, Dom, DomBuilder};
use futures_signals::signal::{Signal, SignalExt};
use web_sys::ShadowRoot;

use crate::{codes::Codes, state::Classroom};

impl Component<Classroom> for Rc<Classroom> {
    fn styles() -> &'static str {
        include_str!("./styles.css")
    }

    fn dom(&self, dom: DomBuilder<ShadowRoot>) -> DomBuilder<ShadowRoot> {
        dom.child(html!("main", {
            .child(PageHeader::new(PageHeaderConfig::default()).render())
            .child_signal(self.page_signal())
            .child(page_footer::dom::render(None))
            .child(OverlayContainer::new().render(None))
        }))
    }
}

impl Classroom {
    fn route_signal(self: &Rc<Self>) -> impl Signal<Item = Route> {
        dominator::routing::url().signal_ref(|url| Route::from_url(url))
    }

    fn page_signal(self: &Rc<Self>) -> impl Signal<Item = Option<Dom>> {
        self.route_signal().map(|route| match route {
            Route::Classroom(route) => Some(match route {
                ClassroomRoute::Codes(route) => Codes::new(route).render(),
            }),
            _ => None,
        })
    }
}
