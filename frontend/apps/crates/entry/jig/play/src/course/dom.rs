use components::{player_popup::{PlayerPopup, PreviewPopupCallbacks}, module::_common::thumbnail::{ModuleThumbnail, ThumbnailFallback}};
use dominator::{html, Dom, clone};
use futures_signals::signal::SignalExt;
use utils::{events, jig::JigPlayerOptions};
use std::rc::Rc;

use super::state::CoursePlayer;

impl CoursePlayer {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;
        state.load_course();

        html!("div", {
            .children_signal_vec(state.jigs.signal_ref(clone!(state => move |jigs| {
                jigs.iter().map(|jig| {
                    let jig_id = jig.id.clone();
                    html!("button", {
                        .style("display", "block")
                        .children(&mut [
                            html!("p", {
                                .text(&jig.jig_data.display_name)
                            }),
                            ModuleThumbnail::new(
                                jig_id.into(),
                                jig.jig_data.modules.first().cloned(),
                                ThumbnailFallback::Asset
                            ).render(None),
                        ])
                        .event(clone!(state, jig_id => move |_: events::Click| {
                            state.active_jig.set(Some(jig_id));
                        }))
                    })
                }).collect()
            })).to_signal_vec())
            .child_signal(state.active_jig.signal_cloned().map(|active_jig| {
                active_jig.map(|active_jig| {
                    html!("div", {
                        .text(&active_jig.0.to_string())
                    })
                })
            }))
            .child_signal(state.active_jig.signal_cloned().map(clone!(state => move|active_jig| {
                active_jig.map(|jig_id| {
                    let close = clone!(state => move || {
                        state.active_jig.set(None);
                    });
                    PlayerPopup::new(
                        jig_id,
                        JigPlayerOptions::default(),
                        PreviewPopupCallbacks::new(close)
                    ).render(None)
                })
            })))
        })
    }
}
