use crate::{course::state::CoursePlayer, jig::state::JigPlayer, playlist::state::PlaylistPlayer};
use components::overlay::container::OverlayContainer;
use dominator::{html, Dom};
use futures_signals::signal::{Signal, SignalExt};
use utils::{
    component::Component,
    routes::{AssetRoute, Route},
};

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
            Route::Asset(AssetRoute::Play(route)) => Some(match route {
                utils::routes::AssetPlayRoute::Jig(jig_id, module_id, player_options) => {
                    JigPlayer::new(jig_id, module_id, player_options).render()
                }
                utils::routes::AssetPlayRoute::Playlist(playlist_id, player_options) => {
                    PlaylistPlayer::new(playlist_id, player_options).render()
                }
                utils::routes::AssetPlayRoute::Course(course_id, unit_id, player_options) => {
                    CoursePlayer::new(course_id, unit_id, player_options).render()
                }
            }),
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
