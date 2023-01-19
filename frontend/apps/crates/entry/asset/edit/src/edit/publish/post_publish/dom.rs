use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use shared::domain::asset::AssetType;
use utils::{
    asset::{AssetPlayerOptions, CoursePlayerOptions, JigPlayerOptions},
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
                    AssetType::Course => "Course",
                    AssetType::Resource => "Resource",
                    AssetType::ProDev => todo!()
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
                    AssetType::Course => {
                        dom.children(state.render_course_actions())
                    },
                    AssetType::ProDev => todo!()
                }
            }))
        })
    }

    fn render_jig_actions(self: &Rc<Self>) -> Vec<Dom> {
        let state = self;
        let share_anchor = html!("post-publish-action", {
            .prop("kind", "share")
            .prop_signal("active", state.share_state.active_popup.signal_cloned().map(|active| active.is_some()))
        });

        vec![
            Rc::clone(&state.share_state).render(share_anchor, Some("actions")),
            html!("post-publish-action", {
                .prop("slot", "actions")
                .prop("kind", "new-jig")
                .event(clone!(state => move |_: events::Click| {
                    state.create_jig();
                }))
            }),
            html!("post-publish-action", {
                .prop("kind", "play-jig")
                .prop("slot", "actions")
                .event(clone!(state => move |_: events::Click| {
                    let settings = AssetPlayerOptions::Jig(JigPlayerOptions::default());
                    state.asset_edit_state.play_jig.set(Some(settings));
                }))
            }),
        ]
    }

    fn render_resource_actions(self: &Rc<Self>) -> Vec<Dom> {
        let state = self;
        vec![
            html!("post-publish-action", {
                .prop("slot", "actions")
                .prop("kind", "new-resource")
                .event(clone!(state => move |_: events::Click| {
                    state.create_resource();
                }))
            }),
            html!("post-publish-action", {
                .prop("kind", "view-resources")
                .prop("slot", "actions")
                .event(|_: events::Click| {
                    Route::Asset(AssetRoute::ResourceGallery).redirect();
                })
            }),
        ]
    }

    fn render_course_actions(self: &Rc<Self>) -> Vec<Dom> {
        let state = self;
        let share_anchor = html!("post-publish-action", {
            .prop("kind", "share")
            .prop_signal("active", state.share_state.active_popup.signal_cloned().map(|active| active.is_some()))
        });

        vec![
            Rc::clone(&state.share_state).render(share_anchor, Some("actions")),
            html!("post-publish-action", {
                .prop("slot", "actions")
                .prop("kind", "new-course")
                .event(clone!(state => move |_: events::Click| {
                    state.create_course();
                }))
            }),
            html!("post-publish-action", {
                .prop("kind", "play-course")
                .prop("slot", "actions")
                .event(clone!(state => move |_: events::Click| {
                    let settings = AssetPlayerOptions::Course(CoursePlayerOptions::default());
                    state.asset_edit_state.play_jig.set(Some(settings));
                }))
            }),
        ]
    }
}
