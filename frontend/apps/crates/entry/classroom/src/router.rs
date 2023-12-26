use components::overlay::container::OverlayContainer;
use utils::{component::Component, routes::*};

use dominator::{html, Dom};
use futures_signals::signal::{Signal, SignalExt};

use crate::codes::{jig_code_sessions::CodeSessions, jig_codes::JigCodes, jigs::Jigs};

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
            Route::Classroom(route) => Some(match route {
                ClassroomRoute::Codes(route) => match route {
                    ClassroomCodesRoute::Jigs => Jigs::new().render(),
                    ClassroomCodesRoute::JigCodes(jig_id) => JigCodes::new(jig_id).render(),
                    ClassroomCodesRoute::JigCodeSession(jig_id, code) => {
                        CodeSessions::new(jig_id, code).render()
                    }
                },
            }),
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
