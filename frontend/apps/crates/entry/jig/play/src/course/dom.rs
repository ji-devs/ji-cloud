use components::{
    module::_common::thumbnail::{ModuleThumbnail, ThumbnailFallback},
    player_popup::{PlayerPopup, PreviewPopupCallbacks},
};
use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use shared::domain::{course::CourseResponse, jig::JigResponse};
use std::rc::Rc;
use utils::{events, jig::JigPlayerOptions, languages::Language};

use super::state::CoursePlayer;

impl CoursePlayer {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;
        state.load_course();
        html!("div", {
            .child_signal(state.course.signal_ref(clone!(state => move|course| {
                course.as_ref().map(|course| {
                    state.render_course(&course)
                })
            })))
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

    fn render_course(self: &Rc<Self>, course: &CourseResponse) -> Dom {
        let state = self;
        let language = Language::code_to_display_name(&course.course_data.language);
        html!("jig-play-course-main", {
            .property("name", &course.course_data.display_name)
            .property("description", &course.course_data.description)
            .property("language", language)
            .property("author", &course.author_name.to_owned().unwrap_or_default())
            .property("itemsCount", course.course_data.items.len())
            .child(
                ModuleThumbnail::new(
                    course.id.into(),
                    course.course_data.cover.clone(),
                    ThumbnailFallback::Asset
                ).render(Some("thumbnail"))
            )
            .children_signal_vec(state.jigs.signal_ref(clone!(state => move |jigs| {
                jigs.iter().enumerate().map(clone!(state => move |(i, jig)| {
                    state.render_item(jig, i)
                })).collect()
            })).to_signal_vec())
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
        html!("jig-play-course-item", {
            .property("slot", "items")
            .property("name", &jig.jig_data.display_name)
            .property("description", &jig.jig_data.description)
            .property("index", i + 1)
            .child(
                ModuleThumbnail::new(
                    jig_id.into(),
                    jig.jig_data.cover.clone(),
                    ThumbnailFallback::Asset
                ).render(Some("thumbnail"))
            )
            .child(html!("fa-button", {
                .property("slot", "play-button")
                .property("icon", "fa-solid fa-play")
            }))
            .event(clone!(state, jig_id => move |_: events::Click| {
                state.active_jig.set(Some(jig_id));
            }))
        })
    }
}
