use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use shared::domain::asset::Asset;
use utils::{
    asset::{AssetPlayerOptions, JigPlayerOptions},
    events,
    routes::{AssetRoute, Route},
};

use super::state::*;
use std::rc::Rc;

impl PostPublish {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;
        html!("post-publish", {
            .property("slot", "main")
            .apply(clone!(state => move |dom| {
                match state.asset {
                    Asset::Resource(_) => {
                        dom.children(state.render_resource_actions())
                    },
                    Asset::Jig(_) => {
                        dom.children(state.render_jig_actions())
                    },
                    Asset::Course(_) => {
                        dom.children(state.render_course_actions())
                    },
                }
            }))
        })
    }

    fn render_jig_actions(self: &Rc<Self>) -> Vec<Dom> {
        let state = self;
        let share_anchor = html!("post-publish-action", {
            .property("kind", "share")
            .property_signal("active", state.share_state.active_popup.signal_cloned().map(|active| active.is_some()))
        });

        vec![
            Rc::clone(&state.share_state).render(share_anchor, Some("actions")),
            html!("post-publish-action", {
                .property("slot", "actions")
                .property("kind", "new-jig")
                .event(clone!(state => move |_: events::Click| {
                    state.create_jig();
                }))
            }),
            html!("post-publish-action", {
                .property("kind", "play-jig")
                .property("slot", "actions")
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
                .property("slot", "actions")
                .property("kind", "new-resource")
                .event(clone!(state => move |_: events::Click| {
                    state.create_resource();
                }))
            }),
            html!("post-publish-action", {
                .property("kind", "view-resources")
                .property("slot", "actions")
                .event(|_: events::Click| {
                    Route::Asset(AssetRoute::ResourceGallery).redirect();
                })
            }),
        ]
    }

    fn render_course_actions(self: &Rc<Self>) -> Vec<Dom> {
        let state = self;
        let share_anchor = html!("post-publish-action", {
            .property("kind", "share")
            .property_signal("active", state.share_state.active_popup.signal_cloned().map(|active| active.is_some()))
        });

        vec![
            Rc::clone(&state.share_state).render(share_anchor, Some("actions")),
            html!("post-publish-action", {
                .property("slot", "actions")
                .property("kind", "new-jig")
                .event(clone!(state => move |_: events::Click| {
                    state.create_course();
                }))
            }),
            html!("post-publish-action", {
                .property("kind", "play-jig")
                .property("slot", "actions")
                .event(clone!(state => move |_: events::Click| {
                    let settings = AssetPlayerOptions::Jig(JigPlayerOptions::default());
                    state.asset_edit_state.play_jig.set(Some(settings));
                }))
            }),
        ]
    }
}
