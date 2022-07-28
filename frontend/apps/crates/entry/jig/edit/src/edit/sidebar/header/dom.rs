use dominator::{clone, html, with_node, Dom};
use futures_signals::signal::SignalExt;
use shared::domain::{
    asset::{Asset, DraftOrLive},
    jig::JigFocus,
};
use std::rc::Rc;
use web_sys::HtmlInputElement;

use crate::edit::sidebar::{jig::actions::get_player_settings, state::SidebarSetting};

use super::super::{
    actions as sidebar_actions, course::settings as course_settings, jig::settings as jig_settings,
    state::State as SidebarState,
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
        html!("jig-edit-sidebar-header", {
            .property("slot", "header")
            .property_signal("collapsed", sidebar_state.collapsed.signal())
            .property_signal("isModulePage", sidebar_state.asset_edit_state.route.signal_cloned().map(|route| {
                // TODO: change?
                matches!(route, AssetEditRoute::Jig(_, _, JigEditRoute::Landing))
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
                    .property("slot", "close")
                    .event(clone!(sidebar_state => move |_: events::Click| {
                        let mut collapsed = sidebar_state.collapsed.lock_mut();
                        *collapsed = !*collapsed;
                    }))
                }),
                html!("button-rect", {
                    .property("slot", "gallery")
                    .property("kind", "text")
                    .property("color", "blue")
                    .property("weight", "medium")
                    .text(STR_MY_JIGS)
                    .event(|_:events::Click| {
                        let url:String = Route::Asset(AssetRoute::JigGallery).into();
                        dominator::routing::go_to_url(&url);
                    })
                }),
                html!("input-wrapper", {
                    .property("slot", "input")
                    .child(html!("input" => HtmlInputElement, {
                        .with_node!(input => {
                            .property("placeholder", STR_SEARCH_PLACEHOLDER)
                            .property_signal("value", sidebar_state.name.signal_cloned())
                            .event(clone!(sidebar_state => move |_: events::Input| {
                                let value = input.value();
                                sidebar_actions::update_display_name(sidebar_state.clone(), value);
                            }))
                        })
                    }))
                    .child(html!("img-ui", {
                        .property("slot", "icon")
                        .property("path", "core/inputs/pencil-blue-darker.svg")
                    }))
                }),
                html!("jig-edit-sidebar-preview-button", {
                    .property("slot", "preview")
                    .event(clone!(sidebar_state => move |_: events::Click| {
                        match &sidebar_state.settings {
                            SidebarSetting::Jig(jig) => {
                                let settings = get_player_settings(Rc::clone(jig));
                                let settings = AssetPlayerOptions::Jig(settings);
                                sidebar_state.asset_edit_state.play_jig.set(Some(settings));
                            },
                            SidebarSetting::Course(_course) => {
                                let settings = CoursePlayerOptions {
                                    draft_or_live: DraftOrLive::Draft
                                };
                                let settings = AssetPlayerOptions::Course(settings);
                                sidebar_state.asset_edit_state.play_jig.set(Some(settings));
                            }
                        }
                    }))
                }),
            ])
            .apply(clone!(sidebar_state => move|dom| {
                if let Asset::Jig(_) = &sidebar_state.asset {
                    dom.child(html!("fa-button", {
                        .property("slot", "modules")
                        .property("icon", "fa-light fa-grid")
                        .event(clone!(sidebar_state => move |_:events::Click| {
                            sidebar_state.asset_edit_state.set_route_jig(JigEditRoute::Landing);
                            let jig = sidebar_state.asset.unwrap_jig();
                            let url = Route::Asset(AssetRoute::Edit(AssetEditRoute::Jig(
                                jig.id,
                                JigFocus::Modules,
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
