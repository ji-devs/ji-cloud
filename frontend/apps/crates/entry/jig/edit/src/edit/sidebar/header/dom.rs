use dominator::{Dom, clone, html, with_node};
use web_sys::HtmlInputElement;
use std::rc::Rc;
use futures_signals::signal::SignalExt;
use crate::edit::sidebar::actions::player_settings_change_signal;

use super::super::{state::State as SidebarState, actions as sidebar_actions, settings};
use utils::prelude::*;

const STR_MY_JIGS:&'static str = "My JIGs";
const STR_SEARCH_PLACEHOLDER:&'static str = "My JIG’s name";

pub struct HeaderDom {
}

impl HeaderDom {
    pub fn render(sidebar_state: Rc<SidebarState>) -> Dom {
        html!("jig-edit-sidebar-header", {
            .property("slot", "header")
            .property_signal("collapsed", sidebar_state.collapsed.signal())
            .property_signal("isModulePage", sidebar_state.route.signal_cloned().map(|route| {
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
                html!("jig-edit-sidebar-action-button", {
                    .property("slot", "modules")
                    .property("kind", "modules")
                    .event(clone!(sidebar_state => move |_:events::Click| {
                        sidebar_state.route.set_neq(JigEditRoute::Landing);
                        let url:String = Route::Jig(JigRoute::Edit(sidebar_state.jig.id.clone(), JigEditRoute::Landing)).into();
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
                    .property_signal("href", player_settings_change_signal(Rc::clone(&sidebar_state)).map(clone!(sidebar_state => move|player_settings| {
                        let route: String = Route::Jig(JigRoute::Play(sidebar_state.jig.id, None, player_settings)).into();
                        route
                    })))
                }),
            ])
        })
    }
}
/*
    <jig-edit-sidebar-header slot="header">
        <button-icon slot="close" icon="x"></button-icon>
        <button-text slot="gallery" color="blue" weight="medium">${STR_MY_JIGS}</button-text>
        <input-text-pencil slot="input"></input-text-pencil>
    </jig-edit-sidebar-header>
    */
