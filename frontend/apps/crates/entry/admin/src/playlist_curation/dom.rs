use std::rc::Rc;

use crate::playlist_curation::details::state::PlaylistDetails;
use dominator::{clone, html, Dom};
use futures_signals::signal::{from_future, SignalExt};
use utils::routes::AdminPlaylistCurationRoute;

use crate::playlist_curation::table::state::PlaylistTable;

use super::PlaylistCuration;

impl PlaylistCuration {
    pub fn render(self: &Rc<Self>) -> Dom {
        let state = self;

        state.load_data();

        html!("empty-fragment", {
            .child(html!("window-loader-block", {
                .prop("slot", "loader")
                .prop_signal("visible", state.loader.is_loading())
            }))
            .child_signal(self.route.signal_ref(clone!(state => move|route| {
                Some(match route {
                    AdminPlaylistCurationRoute::Table => {
                        PlaylistTable::new(
                            Rc::clone(&state)
                        ).render()
                    },
                    AdminPlaylistCurationRoute::Playlist(playlist_id) => {
                        html!("empty-fragment", {
                            .child_signal(from_future(state.clone().get_playlist(*playlist_id)).map(clone!(state => move|playlist| {
                                playlist.map(|playlist| {
                                    PlaylistDetails::new(
                                        Rc::clone(&state),
                                        playlist.id,
                                        playlist
                                    ).render()
                                })
                            })))
                        })
                    },
                })
            })))
        })
    }
}
