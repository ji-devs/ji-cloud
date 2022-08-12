use components::{
    module::_common::thumbnail::{ModuleThumbnail, ThumbnailFallback},
    player_popup::{PlayerPopup, PreviewPopupCallbacks},
};
use dominator::{clone, html, Dom};
use futures_signals::signal::{Signal, SignalExt};
use shared::domain::{course::CourseResponse, jig::JigResponse, meta::ResourceTypeId};
use std::rc::Rc;
use utils::{asset::ResourceContentExt, events, languages::Language};

use super::state::CoursePlayer;

impl CoursePlayer {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;
        state.load_data();
        html!("div", {
            .child_signal(state.course.signal_ref(clone!(state => move|course| {
                course.as_ref().map(|course| {
                    state.render_course(course)
                })
            })))
            .child_signal(state.active_jig.signal_cloned().map(clone!(state => move|active_jig| {
                active_jig.map(|jig_id| {
                    let close = clone!(state => move || {
                        state.done_playing_jig();
                    });
                    PlayerPopup::new_default_player_options(
                        jig_id.into(),
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
            .property("hasAdditionalResources", !course.course_data.additional_resources.is_empty())
            .child(
                ModuleThumbnail::new_hight_res(
                    course.id.into(),
                    course.course_data.cover.clone(),
                    ThumbnailFallback::Asset,
                    state.player_options.draft_or_live,
                ).render(Some("thumbnail"))
            )
            .children_signal_vec(state.jigs.signal_ref(clone!(state => move |jigs| {
                jigs.iter().enumerate().map(clone!(state => move |(i, jig)| {
                    state.render_item(jig, i)
                })).collect()
            })).to_signal_vec())
            .children(course.course_data.additional_resources.iter().map(|resource| {
                html!("a", {
                    .property("slot", "additional-resources")
                    .property("target", "_BLANK")
                    .property("title", &resource.display_name)
                    .property("href", resource.resource_content.get_link())
                    .child(html!("fa-icon", {
                        .property("icon", "fa-light fa-file")
                    }))
                    .text(" ")
                    .text_signal(state.resource_type_name_signal(resource.resource_type_id))
                })
            }))
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
                    jig.jig_data.modules.get(0).cloned(),
                    ThumbnailFallback::Asset,
                    state.player_options.draft_or_live,
                ).render(Some("thumbnail"))
            )
            .child(html!("fa-button", {
                .property("slot", "play-button")
                .property("icon", "fa-solid fa-play")
            }))
            .event(clone!(state, jig_id => move |_: events::Click| {
                state.play_jig(jig_id);
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
