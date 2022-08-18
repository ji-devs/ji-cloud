use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use shared::domain::asset::AssetId;
use utils::{
    asset::{AssetPlayerOptions, JigPlayerOptions},
    events,
    routes::{AssetRoute, Route},
};

use super::{actions, state::*};
use std::rc::Rc;

impl PostPublish {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;
        html!("post-publish", {
            .property("slot", "main")
            .apply(clone!(state => move |dom| {
                match state.asset_edit_state.asset_id {
                    AssetId::ResourceId(_) => {
                        dom.children(
                            render_resources_focused_actions(&state)
                        )
                    },
                    AssetId::JigId(_) => {
                        dom.children(
                            render_modules_focused_actions(&state)
                        )
                    },
                    AssetId::CourseId(_) => todo!(),
                }
            }))
        })
    }
}

fn render_modules_focused_actions(state: &Rc<PostPublish>) -> Vec<Dom> {
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
                actions::create_jig(Rc::clone(&state));
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

fn render_resources_focused_actions(state: &Rc<PostPublish>) -> Vec<Dom> {
    vec![
        html!("post-publish-action", {
            .property("slot", "actions")
            .property("kind", "new-resource")
            .event(clone!(state => move |_: events::Click| {
                actions::create_jig(Rc::clone(&state));
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
