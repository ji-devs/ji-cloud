use dominator::{clone, html, with_node, Dom};
use futures_signals::signal::SignalExt;
use shared::domain::asset::{AssetId, DraftOrLive};
use std::rc::Rc;
use web_sys::HtmlInputElement;

use super::super::{actions as sidebar_actions, state::Sidebar as SidebarState};
use crate::edit::sidebar::{jig::actions::get_player_settings, state::SidebarSetting};
use shared::domain::asset::AssetType;
use utils::{
    asset::{AssetPlayerOptions, CoursePlayerOptions, ProDevPlayerOptions},
    prelude::*,
};

const STR_MY_JIGS_1: &str = "My ";
const STR_MY_JIGS_2: &str = "s";
const STR_SEARCH_PLACEHOLDER_1: &str = "My ";
const STR_SEARCH_PLACEHOLDER_2: &str = "’s name";

pub struct HeaderDom {}

impl HeaderDom {
    pub fn render(sidebar_state: Rc<SidebarState>) -> Dom {
        let asset_edit_state = Rc::clone(&sidebar_state.asset_edit_state);
        html!("jig-edit-sidebar-header", {
            .prop("slot", "header")
            // TODO: remove once course has setting
            .prop("hasSettings", (!asset_edit_state.asset_id.is_course_id() && !asset_edit_state.asset_id.is_pro_dev_id()))
            .prop_signal("collapsed", sidebar_state.collapsed.signal())
            .prop_signal("isModulePage", asset_edit_state.route.signal_cloned().map(|route| {
                // TODO: change?
                matches!(
                    route,
                    AssetEditRoute::Jig(_, JigEditRoute::Landing) | AssetEditRoute::Course(_, CourseEditRoute::Landing)
                )
            }))
            .apply(|dom| {
                match &sidebar_state.settings {
                    SidebarSetting::Jig(settings) => {
                        dom.child(settings.render())
                    },
                    SidebarSetting::Course(settings) => {
                        dom.child(settings.render())
                    },
                    SidebarSetting::ProDev(settings) => {
                        dom.child(settings.render())
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
                    .text(&format!("{}{}{}",
                        STR_MY_JIGS_1,
                        asset_edit_state.asset.asset_type().sidebar_header(),
                        STR_MY_JIGS_2
                    ))
                    .event(clone!(asset_edit_state => move |_:events::Click| {
                        let route = match asset_edit_state.asset_id {
                            AssetId::JigId(_) => AssetRoute::JigGallery,
                            AssetId::CourseId(_) => AssetRoute::CourseGallery,
                            AssetId::ResourceId(_) => unimplemented!(),
                            AssetId::ProDevId(_) => AssetRoute::ProDevGallery,
                        };
                        let url:String = Route::Asset(route).into();
                        dominator::routing::go_to_url(&url);
                    }))
                }),
                html!("input-wrapper", {
                    .prop("slot", "input")
                    .child(html!("input" => HtmlInputElement, {
                        .with_node!(input => {
                            .prop("placeholder", format!("{}{}{}",
                                STR_SEARCH_PLACEHOLDER_1,
                                asset_edit_state.asset.asset_type().display_name(),
                                STR_SEARCH_PLACEHOLDER_2
                            ))
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
                    .prop("assetDisplayName", {
                        let asset_type = asset_edit_state.asset.asset_type();
                        if asset_type != AssetType::ProDev {
                            asset_type.display_name()
                        } else {
                            "course"
                        }
                    })
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
                            SidebarSetting::ProDev(_pro_dev) =>
                            {
                                let settings = ProDevPlayerOptions {
                                    draft_or_live: DraftOrLive::Draft,
                                    is_student: false
                                };
                                let settings = AssetPlayerOptions::ProDev(settings);
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
