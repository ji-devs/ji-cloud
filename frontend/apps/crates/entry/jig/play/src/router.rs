use crate::player::{dom::render as player_render, state::State as PlayerState};
use components::overlay::container::OverlayContainer;
use dominator::{html, Dom};
use futures_signals::signal::{Signal, SignalExt};
use std::rc::Rc;
use utils::routes::{JigRoute, Route};

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
            Route::Jig(route) => match route {
                JigRoute::Play(jig_id, module_id, player_options) => Some(player_render(Rc::new(
                    PlayerState::new(jig_id, module_id, player_options),
                ))),
                _ => None,
            },
            _ => None,
        })
    }

    pub fn render(&self) -> Dom {
        html!("main", {
            .child_signal(Self::dom_signal())
            .child(OverlayContainer::new().render(None))
        } )
    }
}
