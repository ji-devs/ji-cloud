use dominator::{clone, html, with_node, Dom};
use futures_signals::signal::SignalExt;
use shared::domain::asset::{AssetId, DraftOrLive};
use std::rc::Rc;
use web_sys::HtmlInputElement;

use crate::edit::sidebar::{jig::actions::get_player_settings, state::SidebarSetting};

use super::super::{
    actions as sidebar_actions, course::settings as course_settings, jig::settings as jig_settings,
    state::Sidebar as SidebarState,
};
use utils::{
    asset::{AssetPlayerOptions, CoursePlayerOptions},
    prelude::*,
};

const STR_MY_JIGS: &str = "My JIGs";
const STR_SEARCH_PLACEHOLDER: &str = "My JIG’s name";

pub struct HeaderDom {}

impl HeaderDom {
    pub fn render(sidebar_state: Rc<SidebarState>) -> Dom {
        let asset_edit_state = Rc::clone(&sidebar_state.asset_edit_state);
        html!("jig-edit-sidebar-header", {
            .prop("slot", "header")
            .prop_signal("collapsed", sidebar_state.collapsed.signal())
            .prop_signal("isModulePage", asset_edit_state.route.signal_cloned().map(|route| {
                // TODO: change?
                matches!(route, AssetEditRoute::Jig(_, JigEditRoute::Landing))
            }))
            .apply(|dom| {
                match &sidebar_state.settings {
                    SidebarSetting::Jig(settings) => {
                        dom.child(jig_settings::dom::render(Rc::clone(settings)))
                    },
                    SidebarSetting::Course(settings) => {
                        dom.child(course_settings::dom::render(Rc::clone(settings)))
                    },
                }
            })
            .children(&mut [
                html!("jig-edit-sidebar-close-button", {
                    .prop("slot", "close")
                    .event(clone!(sidebar_state => move |_: events::Click| {
                        let mut collapsed = sidebar_state.collapsed.lock_mut();
                        *collapsed = !*collapsed;
                    }))
                }),
                html!("button-rect", {
                    .prop("slot", "gallery")
                    .prop("kind", "text")
                    .prop("color", "blue")
                    .prop("weight", "medium")
                    .text(STR_MY_JIGS)
                    .event(|_:events::Click| {
                        let url:String = Route::Asset(AssetRoute::JigGallery).into();
                        dominator::routing::go_to_url(&url);
                    })
                }),
                html!("input-wrapper", {
                    .prop("slot", "input")
                    .child(html!("input" => HtmlInputElement, {
                        .with_node!(input => {
                            .prop("placeholder", STR_SEARCH_PLACEHOLDER)
                            .prop_signal("value", asset_edit_state.asset.display_name().signal_cloned())
                            .event(clone!(sidebar_state => move |_: events::Input| {
                                let value = input.value();
                                sidebar_actions::update_display_name(sidebar_state.clone(), value);
                            }))
                        })
                    }))
                    .child(html!("img-ui", {
                        .prop("slot", "icon")
                        .prop("path", "core/inputs/pencil-blue-darker.svg")
                    }))
                }),
                html!("jig-edit-sidebar-preview-button", {
                    .prop("slot", "preview")
                    .event(clone!(sidebar_state, asset_edit_state => move |_: events::Click| {
                        match &sidebar_state.settings {
                            SidebarSetting::Jig(jig) => {
                                let settings = get_player_settings(Rc::clone(jig));
                                let settings = AssetPlayerOptions::Jig(settings);
                                asset_edit_state.play_jig.set(Some(settings));
                            },
                            SidebarSetting::Course(_course) => {
                                let settings = CoursePlayerOptions {
                                    draft_or_live: DraftOrLive::Draft,
                                    is_student: false
                                };
                                let settings = AssetPlayerOptions::Course(settings);
                                asset_edit_state.play_jig.set(Some(settings));
                            }
                        }
                    }))
                }),
            ])
            .apply(clone!(asset_edit_state => move|dom| {
                if let AssetId::JigId(jig_id) = asset_edit_state.asset_id {
                    dom.child(html!("fa-button", {
                        .prop("slot", "modules")
                        .prop("icon", "fa-light fa-grid")
                        .event(clone!(asset_edit_state => move |_:events::Click| {
                            asset_edit_state.set_route_jig(JigEditRoute::Landing);
                            let url = Route::Asset(AssetRoute::Edit(AssetEditRoute::Jig(
                                jig_id,
                                JigEditRoute::Landing
                            ))).to_string();
                            dominator::routing::go_to_url(&url);
                        }))
                    }))
                } else {
                    dom
                }
            }))
        })
    }
}
