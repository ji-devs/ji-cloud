use std::{rc::Rc, sync::Arc};

use crate::pro_dev::player_popup::PlayerPopup;

use super::PlayerMain;
use components::{
    module::_common::thumbnail::{ModuleThumbnail, ThumbnailFallback},
    share_asset::ShareAsset,
};
use dominator::{clone, html, Dom, DomBuilder};
use futures_signals::signal::{from_future, Signal, SignalExt};
use shared::domain::{
    meta::{ResourceType, ResourceTypeId},
    pro_dev::{unit::ProDevUnit, ProDevResponse},
};
use utils::{
    asset::ResourceContentExt, component::Component, events, languages::Language,
    metadata::get_resource_types,
};
use web_sys::ShadowRoot;

const STR_SHARE_COURSE: &str = "Share course";
const UNITS_PER_PAGE: usize = 10;

impl Component<PlayerMain> for Rc<PlayerMain> {
    fn styles() -> &'static str {
        include_str!("./styles.css")
    }

    fn dom(&self, dom: DomBuilder<ShadowRoot>) -> DomBuilder<ShadowRoot> {
        let state = self;
        dom.child(html!("div", {
            .child_signal(state.player_state.pro_dev.signal_cloned().map(clone!(state => move |pro_dev| {
                if let Some(pro_dev) = pro_dev {
                    Some(state.render_pro_dev_landing(&pro_dev))
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
    fn render_pro_dev_landing(self: &Rc<Self>, pro_dev: &Rc<ProDevResponse>) -> Dom {
        let state = self;
        let language = Language::code_to_display_name(&pro_dev.pro_dev_data.language);

        html!("jig-play-course-main", {
            .prop("name", &pro_dev.pro_dev_data.display_name)
            .prop("description", &pro_dev.pro_dev_data.description)
            .prop("language", language)
            .prop("author", &pro_dev.author_name.to_owned().unwrap_or_default())
            .prop("itemsCount", pro_dev.pro_dev_data.units.len())
            .prop("hasAdditionalResources", !pro_dev.pro_dev_data.additional_resources.is_empty())
            .child(
                ModuleThumbnail::new_hight_res(
                    pro_dev.id.into(),
                    pro_dev.pro_dev_data.cover.clone(),
                    ThumbnailFallback::Asset,
                    state.player_state.player_options.draft_or_live,
                ).render(Some("thumbnail"))
            )
            .children_signal_vec(state.player_state.pro_dev.signal_cloned().map(clone!(state => move |pro_dev| {
                match pro_dev {
                    Some(pro_dev) => {
                        pro_dev.pro_dev_data.units.iter().enumerate().map(clone!(state => move |(i, unit)| {
                            state.render_unit(unit, i)
                        })).collect()
                    }
                    None => todo!()
                }
            })).to_signal_vec())
            .children(pro_dev.pro_dev_data.additional_resources.iter().map(|resource| {
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
            .child(html!("fa-button", {
                .prop("slot", "play")
                .prop("icon", "fa-solid fa-circle-play")
                .event(clone!(state => move |_: events::Click| {
                    state.player_state.active_unit.set(Some(0));
                    state.player_state.played_units.lock_mut().insert(0);
                }))
            }))
            .child(ShareAsset::new(pro_dev.as_ref().clone().into()).render(
                html!("button-empty", {
                    .child(html!("fa-icon", {
                        .prop("icon", "fa-light fa-share-nodes")
                    }))
                    .text(STR_SHARE_COURSE)
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

    fn render_unit(self: &Rc<Self>, unit: &ProDevUnit, i: usize) -> Dom {
        let state = self;
        html!("jig-play-course-item", {
            .prop("slot", "items")
            .prop("name", &unit.display_name)
            .prop("description", &unit.description)
            .prop("index", i + 1)
            .prop_signal("done", state.player_state.played_units.signal_ref(move |played_units| played_units.contains(&i)))
            .child(html!("fa-button", {
                .prop("slot", "play-button")
                .prop("icon", "fa-solid fa-play")
            }))
            .event(clone!(state => move |_: events::Click| {
                state.set_active_unit_and_update_page(i);
            }))
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
}
