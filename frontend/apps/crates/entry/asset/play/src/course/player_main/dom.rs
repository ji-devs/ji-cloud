use std::{rc::Rc, sync::Arc};

use crate::course::player_popup::PlayerPopup;

use super::PlayerMain;
use components::{
    module::_common::thumbnail::{ModuleThumbnail, ThumbnailFallback},
    share_asset::ShareAsset,
    unit::thumbnail::UnitThumbnail,
};
use dominator::{clone, html, Dom, DomBuilder};
use futures_signals::signal::{from_future, Signal, SignalExt};
use shared::domain::{
    course::{unit::CourseUnit, CourseResponse},
    meta::{ResourceType, ResourceTypeId},
};
use utils::{
    asset::ResourceContentExt,
    component::Component,
    events,
    js_wrappers::is_iframe,
    languages::Language,
    metadata::get_resource_types,
    prelude::{AssetPlayerToPlayerPopup, IframeAction, IframeMessageExt},
    unwrap::UnwrapJiExt,
};
use web_sys::ShadowRoot;

const STR_SHARE_PLAYLIST: &str = "Share playlist";
const UNITS_PER_PAGE: usize = 10;

impl Component<PlayerMain> for Rc<PlayerMain> {
    fn styles() -> &'static str {
        include_str!("./styles.css")
    }

    fn dom(&self, dom: DomBuilder<ShadowRoot>) -> DomBuilder<ShadowRoot> {
        let state = self;
        dom.child(html!("div", {
            .child_signal(state.player_state.course.signal_cloned().map(clone!(state => move |course| {
                if let Some(course) = course {
                    Some(state.render_course_landing(&course))
                } else {
                    None
                }
            })))
            .child_signal(state.player_state.active_unit.signal().map(clone!(state => move |active_unit| {
                active_unit.map(|_unit_id| {
                    PlayerPopup::new(&state.player_state).render()
                })
            })))

        }))
    }
}

impl PlayerMain {
    fn render_course_landing(self: &Rc<Self>, course: &Rc<CourseResponse>) -> Dom {
        let state = self;
        let language = Language::code_to_display_name(&course.course_data.language);

        html!("jig-play-playlist-main", {
            .prop("name", &course.course_data.display_name)
            .prop("description", &course.course_data.description)
            .prop("language", language)
            .prop("author", &course.author_name.to_owned().unwrap_or_default())
            .prop("itemsCount", course.course_data.units.len())
            .prop("itemType", "Units")
            .prop("hasAdditionalResources", !course.course_data.additional_resources.is_empty())
            .child(
                ModuleThumbnail::new_hight_res(
                    course.id.into(),
                    course.course_data.cover.clone(),
                    ThumbnailFallback::Asset,
                    state.player_state.player_options.draft_or_live,
                ).render(Some("thumbnail"))
            )
            .children_signal_vec(state.player_state.course.signal_cloned().map(clone!(state => move |course| {
                match course {
                    Some(course) => {
                        course.course_data.units.iter().enumerate().map(clone!(state => move |(i, unit)| {
                            state.render_unit(unit, i)
                        })).collect()
                    }
                    None => todo!()
                }
            })).to_signal_vec())
            .children(course.course_data.additional_resources.iter().map(|resource| {
                html!("a", {
                    .prop("slot", "additional-resources")
                    .prop("target", "_BLANK")
                    .prop("title", &resource.display_name)
                    .prop("href", resource.resource_content.get_link())
                    .child(html!("fa-icon", {
                        .prop("icon", "fa-light fa-file")
                    }))
                    .text(" ")
                    .text_signal(state.resource_name_signal(resource.resource_type_id))
                })
            }))
            .child(html!("div", {
                .prop("slot", "play")
                .child(html!("fa-button", {
                    .prop("icon", "fa-solid fa-circle-play")
                }))
                .event(clone!(state => move |_: events::Click| {
                    state.render_popup.set(false);
                    state.player_state.active_unit.set(Some(0));
                    state.player_state.played_units.lock_mut().insert(0);
                }))
            }))
            .child(ShareAsset::new(course.as_ref().clone().into()).render(
                html!("button-empty", {
                    .child(html!("fa-icon", {
                        .prop("icon", "fa-light fa-share-nodes")
                    }))
                    .text(STR_SHARE_PLAYLIST)
                }),
                Some("share")
            ))
            // .child_signal(state.active_unit.signal_cloned().map(|active_unit| {
            //     active_unit.map(|active_unit| {
            //         html!("div", {
            //             .text(&active_unit.0.to_string())
            //         })
            //     })
            // }))
        })
    }

    fn render_unit(self: &Rc<Self>, unit: &CourseUnit, i: usize) -> Dom {
        let state = self;
        html!("jig-play-playlist-item", {
            .prop("slot", "items")
            .prop("name", &unit.display_name)
            .prop("description", &unit.description)
            .prop("hideDescription", true)
            .prop("index", i + 1)
            .prop_signal("done", state.player_state.played_units.signal_ref(move |played_units| played_units.contains(&i)))
            .child(html!("fa-button", {
                .prop("slot", "play-button")
                .prop("icon", "fa-solid fa-play")
                .style("place-content", "center")
                .event(clone!(state => move |_: events::Click| {
                    state.render_popup.set(false);
                    state.set_active_unit_and_update_page(i);
                }))
            }))
            .child(UnitThumbnail::new(
                unit.value.clone(),
            ).render_live(Some("thumbnail")))
            .child(html!("button-empty", {
                .prop("slot", "read-more")
                .text("Read more")
                .event(clone!(state, unit => move |_: events::Click| {
                    state.read_more.set(Some(unit.clone()));
                    state.render_popup.set(true);
                }))
            }))
            .child_signal(state.render_popup.signal_cloned().map(clone!(state, unit => move |render_popup| {
                match render_popup {
                    true => {
                        let popup = if &state.read_more.get_cloned().unwrap_ji().id == &unit.id {
                            Some(state.render_info_popup(&state.read_more.get_cloned().unwrap_ji()))
                        } else {
                            None
                        };
                        popup
                    },
                    false => None,
                }
            })))

        })
    }

    pub fn set_active_unit_and_update_page(self: &Rc<Self>, active_unit: usize) {
        let current_page = active_unit / UNITS_PER_PAGE;
        self.player_state.current_page.set(Some(current_page));
        self.player_state.active_unit.set(Some(active_unit));

        self.player_state
            .played_units
            .lock_mut()
            .insert(active_unit);

        if is_iframe() {
            let _ = IframeAction::new(AssetPlayerToPlayerPopup::CloseButtonShown(false))
                .try_post_message_to_parent();
        }
    }

    fn resource_name_signal(
        self: &Rc<Self>,
        resource_type_id: ResourceTypeId,
    ) -> impl Signal<Item = String> {
        from_future(get_resource_types()).map(
            move |resource_types: Option<Arc<Vec<ResourceType>>>| match resource_types {
                Some(resource_types) => resource_types
                    .iter()
                    .find(move |t| t.id == resource_type_id)
                    .map(|t| t.display_name.clone())
                    .unwrap_or_default(),
                None => String::new(),
            },
        )
    }

    pub fn render_info_popup(self: &Rc<Self>, unit: &CourseUnit) -> Dom {
        let state = self;
        log::warn!("poup: {}", unit.display_name);
        html!("main-popup-info", {
            .prop("slot", "popup-info")
            .children(&mut [
                html!("fa-button", {
                    .prop("icon", "fa-light fa-xmark")
                    .event(clone!(state => move |_: events::Click| {
                        state.render_popup.set(false);
                        state.read_more.set(None);
                    }))
                }),
                html!("div", {
                    .class("popup-name")
                    .text(&unit.display_name)
                }),
                html!("div", {
                    .class("popup-description")
                    .text(&unit.description)
                }),
                html!("div", {
                    .class("popup-close")
                    .text("Close")
                    .event(clone!(state => move |_: events::Click| {
                        state.render_popup.set(false);
                        state.read_more.set(None);
                    }))
                })
            ])
        })
    }
}
