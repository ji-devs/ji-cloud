use crate::edit::sidebar::actions::get_player_settings;
use dominator::{clone, html, with_node, Dom};
use futures_signals::signal::SignalExt;
use std::rc::Rc;
use web_sys::HtmlInputElement;

use super::super::{actions as sidebar_actions, settings, state::State as SidebarState};
use utils::prelude::*;

const STR_MY_JIGS: &'static str = "My JIGs";
const STR_SEARCH_PLACEHOLDER: &'static str = "My JIG’s name";

pub struct HeaderDom {}

impl HeaderDom {
    pub fn render(sidebar_state: Rc<SidebarState>) -> Dom {
        html!("jig-edit-sidebar-header", {
            .property("slot", "header")
            .property_signal("collapsed", sidebar_state.collapsed.signal())
            .property_signal("isModulePage", sidebar_state.jig_edit_state.route.signal_cloned().map(|route| {
                matches!(route, JigEditRoute::Landing)
            }))
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
                        let url:String = Route::Jig(JigRoute::Gallery).into();
                        dominator::routing::go_to_url(&url);
                    })
                }),
                settings::dom::render(Rc::clone(&sidebar_state.settings)),
                html!("fa-button", {
                    .property("slot", "modules")
                    .property("icon", "fa-light fa-grid")
                    .event(clone!(sidebar_state => move |_:events::Click| {
                        sidebar_state.jig_edit_state.route.set_neq(JigEditRoute::Landing);
                        let url:String = Route::Jig(JigRoute::Edit(
                            sidebar_state.jig.id.clone(),
                            sidebar_state.jig.jig_focus,
                            JigEditRoute::Landing
                        )).into();
                        dominator::routing::go_to_url(&url);
                    }))
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
                        let settings = get_player_settings(Rc::clone(&sidebar_state));
                        sidebar_state.jig_edit_state.play_jig.set(Some(settings));
                    }))
                }),
            ])
        })
    }
}
