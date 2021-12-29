use components::overlay::container::OverlayContainer;
use shared::domain::jig::JigFocus;
use utils::routes::{JigRoute, Route};

use crate::{edit::dom::EditPage, gallery::state::JigGallery};
use dominator::{html, Dom};
use futures_signals::signal::{Signal, SignalExt};

pub struct Router {}

impl Router {
    pub fn new() -> Self {
        Self {}
    }

    fn signal() -> impl Signal<Item = Route> {
        dominator::routing::url().signal_ref(|url| Route::from_url(&url))
    }

    fn dom_signal() -> impl Signal<Item = Option<Dom>> {
        Self::signal().map(|route| match route {
            Route::Jig(route) => match route {
                JigRoute::Gallery => Some(
                    JigGallery::new(JigFocus::Modules).render()
                ),
                JigRoute::ResourceGallery => Some(
                    JigGallery::new(JigFocus::Resources).render()
                ),
                JigRoute::Edit(jig_id, jig_focus, route) => Some(
                    EditPage::render(jig_id, jig_focus, route)
                ),
                _ => None,
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
