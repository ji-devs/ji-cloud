use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use shared::domain::asset::AssetType;
use utils::{
    asset::{
        self, AssetPlayerOptions, CoursePlayerOptions, JigPlayerOptions, PlaylistPlayerOptions,
    },
    events,
    routes::{AssetRoute, Route},
};

use super::state::*;
use std::rc::Rc;

impl PostPublish {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;
        html!("post-publish", {
            .prop("slot", "main")
            .apply(clone!(state => move |dom| {
                dom.prop("assetName", match state.asset_edit_state.asset.asset_type() {
                    AssetType::Jig => "JIG",
                    AssetType::Playlist => "Playlist",
                    AssetType::Resource => "Resource",
                    AssetType::Course => "Course"
                })
            }))
            .apply(clone!(state => move |dom| {
                match state.asset_edit_state.asset.asset_type() {
                    AssetType::Resource => {
                        dom.children(state.render_resource_actions())
                    },
                    AssetType::Jig => {
                        dom.children(state.render_jig_actions())
                    },
                    AssetType::Playlist => {
                        dom.children(state.render_playlist_actions())
                    },
                    AssetType::Course => {
                        dom.children(state.render_course_actions())
                    }
                }
            }))
        })
    }

    fn render_jig_actions(self: &Rc<Self>) -> Vec<Dom> {
        let state = self;
        let asset_display_name = AssetType::Jig.display_name();
        let share_anchor = html!("post-publish-action", {
            .prop("kind", "share")
            .prop("assetDisplayName", asset_display_name)
            .prop_signal("active", state.share_state.active_popup.signal_cloned().map(|active| active.is_some()))
        });

        vec![
            Rc::clone(&state.share_state).render(share_anchor, Some("actions")),
            html!("post-publish-action", {
                .prop("slot", "actions")
                .prop("kind", "new")
                .prop("assetDisplayName", asset_display_name)
                .event(move |_: events::Click| {
                    asset::create_jig();
                })
            }),
            html!("post-publish-action", {
                .prop("kind", "play")
                .prop("assetDisplayName", asset_display_name)
                .prop("slot", "actions")
                .event(clone!(state => move |_: events::Click| {
                    let settings = AssetPlayerOptions::Jig(JigPlayerOptions::default());
                    state.asset_edit_state.play_jig.set(Some(settings));
                }))
            }),
        ]
    }

    fn render_resource_actions(self: &Rc<Self>) -> Vec<Dom> {
        let asset_display_name = AssetType::Resource.display_name();
        vec![
            html!("post-publish-action", {
                .prop("slot", "actions")
                .prop("kind", "new")
                .prop("assetDisplayName", asset_display_name)
                .event(move |_: events::Click| {
                    asset::create_resource();
                })
            }),
            html!("post-publish-action", {
                .prop("kind", "view-others")
                .prop("assetDisplayName", asset_display_name)
                .prop("slot", "actions")
                .event(|_: events::Click| {
                    Route::Asset(AssetRoute::ResourceGallery).redirect();
                })
            }),
        ]
    }

    fn render_playlist_actions(self: &Rc<Self>) -> Vec<Dom> {
        let state = self;
        let asset_display_name = AssetType::Playlist.display_name();
        let share_anchor = html!("post-publish-action", {
            .prop("kind", "share")
            .prop("assetDisplayName", asset_display_name)
            .prop_signal("active", state.share_state.active_popup.signal_cloned().map(|active| active.is_some()))
        });

        vec![
            Rc::clone(&state.share_state).render(share_anchor, Some("actions")),
            html!("post-publish-action", {
                .prop("slot", "actions")
                .prop("kind", "new")
                .prop("assetDisplayName", asset_display_name)
                .event(move |_: events::Click| {
                    asset::create_playlist();
                })
            }),
            html!("post-publish-action", {
                .prop("kind", "play")
                .prop("assetDisplayName", asset_display_name)
                .prop("slot", "actions")
                .event(clone!(state => move |_: events::Click| {
                    let settings = AssetPlayerOptions::Playlist(PlaylistPlayerOptions::default());
                    state.asset_edit_state.play_jig.set(Some(settings));
                }))
            }),
        ]
    }

    fn render_course_actions(self: &Rc<Self>) -> Vec<Dom> {
        let state = self;
        let asset_display_name = AssetType::Course.display_name();
        let share_anchor = html!("post-publish-action", {
            .prop("kind", "share")
            .prop("assetDisplayName", asset_display_name)
            .prop_signal("active", state.share_state.active_popup.signal_cloned().map(|active| active.is_some()))
        });

        vec![
            Rc::clone(&state.share_state).render(share_anchor, Some("actions")),
            html!("post-publish-action", {
                .prop("slot", "actions")
                .prop("kind", "new")
                .prop("assetDisplayName", asset_display_name)
                .event(move |_: events::Click| {
                    asset::create_course();
                })
            }),
            html!("post-publish-action", {
                .prop("kind", "play")
                .prop("assetDisplayName", asset_display_name)
                .prop("slot", "actions")
                .event(clone!(state => move |_: events::Click| {
                    let settings = AssetPlayerOptions::Course(CoursePlayerOptions::default());
                    state.asset_edit_state.play_jig.set(Some(settings));
                }))
            }),
        ]
    }
}
