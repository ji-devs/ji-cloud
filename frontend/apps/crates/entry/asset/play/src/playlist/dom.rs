use components::{
    module::_common::thumbnail::{ModuleThumbnail, ThumbnailFallback},
    player_popup::{PlayerPopup, PreviewPopupCallbacks},
    share_asset::ShareAsset,
};
use dominator::{clone, html, Dom};
use futures_signals::signal::{Signal, SignalExt};
use shared::domain::{jig::JigResponse, meta::ResourceTypeId, playlist::PlaylistResponse};
use std::rc::Rc;
use utils::{
    asset::{AssetPlayerOptions, JigPlayerOptions, ResourceContentExt},
    events,
    languages::Language,
};

use super::state::PlaylistPlayer;

const STR_SHARE_PLAYLIST: &str = "Share playlist";

impl PlaylistPlayer {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;
        state.load_data();
        html!("div", {
            .child_signal(state.playlist.signal_ref(clone!(state => move|playlist| {
                playlist.as_ref().map(|playlist| {
                    state.render_playlist(playlist)
                })
            })))
            .child_signal(state.active_jig.signal_cloned().map(clone!(state => move|active_jig| {
                active_jig.map(|jig_id| {
                    let close = clone!(state => move || {
                        state.done_playing_jig();
                    });
                    let options = AssetPlayerOptions::Jig(JigPlayerOptions {
                        is_student: state.player_options.is_student,
                        quota: true,
                        ..Default::default()
                    });
                    PlayerPopup::new(
                        jig_id.into(),
                        None,
                        None,
                        options,
                        PreviewPopupCallbacks::new(close)
                    ).render(None)
                })
            })))
        })
    }

    fn render_playlist(self: &Rc<Self>, playlist: &PlaylistResponse) -> Dom {
        let state = self;
        let language = Language::code_to_display_name(&playlist.playlist_data.language);
        html!("jig-play-playlist-main", {
            .prop("name", &playlist.playlist_data.display_name)
            .prop("description", &playlist.playlist_data.description)
            .prop("language", language)
            .prop("author", &playlist.author_name.to_owned().unwrap_or_default())
            .prop("itemsCount", playlist.playlist_data.items.len())
            .prop("itemType", "Jigs")
            .prop("hasAdditionalResources", !playlist.playlist_data.additional_resources.is_empty())
            .child(
                ModuleThumbnail::new_hight_res(
                    playlist.id.into(),
                    playlist.playlist_data.cover.clone(),
                    ThumbnailFallback::Asset,
                    state.player_options.draft_or_live,
                ).render(Some("thumbnail"))
            )
            .children_signal_vec(state.jigs.signal_ref(clone!(state => move |jigs| {
                jigs.iter().enumerate().map(clone!(state => move |(i, jig)| {
                    state.render_item(jig, i)
                })).collect()
            })).to_signal_vec())
            .children(playlist.playlist_data.additional_resources.iter().map(|resource| {
                html!("a", {
                    .prop("slot", "additional-resources")
                    .prop("target", "_BLANK")
                    .prop("title", &resource.display_name)
                    .prop("href", resource.resource_content.get_link())
                    .child(html!("fa-icon", {
                        .prop("icon", "fa-light fa-file")
                    }))
                    .text(" ")
                    .text_signal(state.resource_type_name_signal(resource.resource_type_id))
                })
            }))
            .child(html!("fa-button", {
                .prop("slot", "play")
                .prop("icon", "fa-solid fa-circle-play")
                .event(clone!(state => move |_: events::Click| {
                    let jigs = state.jigs.lock_mut();
                    let jig_id = jigs.first().map(|jig| jig.id);
                    state.active_jig.set(jig_id);

                }))
            }))
            .child(ShareAsset::new(playlist.clone().into()).render(
                html!("button-empty", {
                    .child(html!("fa-icon", {
                        .prop("icon", "fa-light fa-share-nodes")
                    }))
                    .text(STR_SHARE_PLAYLIST)
                }),
                Some("share")
            ))
            .child_signal(state.active_jig.signal_cloned().map(|active_jig| {
                active_jig.map(|active_jig| {
                    html!("div", {
                        .text(&active_jig.0.to_string())
                    })
                })
            }))
        })
    }

    fn render_item(self: &Rc<Self>, jig: &JigResponse, i: usize) -> Dom {
        let state = self;
        let jig_id = jig.id;
        html!("jig-play-playlist-item", {
            .prop("slot", "items")
            .prop("name", &jig.jig_data.display_name)
            .prop("description", &jig.jig_data.description)
            .prop("index", i + 1)
            .prop("premium", jig.admin_data.premium)
            .prop_signal("done", state.jigs_done.signal_ref(move |jigs_done| jigs_done.contains(&jig_id)))
            .child(
                ModuleThumbnail::new(
                    jig_id.into(),
                    jig.jig_data.modules.get(0).cloned(),
                    ThumbnailFallback::Asset,
                    state.player_options.draft_or_live,
                ).render(Some("thumbnail"))
            )
            .child(html!("fa-button", {
                .prop("slot", "play-button")
                .prop("icon", "fa-solid fa-play")
            }))
            .event(clone!(state, jig_id => move |_: events::Click| {
                state.play_jig(jig_id);
                state.jigs_done.lock_mut().insert(jig_id);
            }))
        })
    }

    fn resource_type_name_signal(
        self: &Rc<Self>,
        resource_type_id: ResourceTypeId,
    ) -> impl Signal<Item = String> {
        let state = Rc::clone(self);

        state
            .resource_types
            .signal_cloned()
            .map(move |resource_types| {
                let resource_type = resource_types
                    .iter()
                    .find(|resource_type| resource_type_id == resource_type.id);

                match resource_type {
                    None => String::new(),
                    Some(resource_type) => resource_type.display_name.to_owned(),
                }
            })
    }
}
