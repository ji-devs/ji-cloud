use dominator::{html, Dom, clone};
use shared::domain::jig::Jig;
use std::rc::Rc;
use futures_signals::signal::SignalExt;
use super::super::{state::State as SidebarState, actions as sidebar_actions};
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
                    .event(|evt:events::Click| {
                        let url:String = Route::Jig(JigRoute::Gallery).into();
                        dominator::routing::go_to_url(&url);
                    })
                }),
                html!("anchored-overlay", {
                    .property("slot", "settings")
                    .property("positionX", "right-out")
                    .property("positionY", "top-in")
                    .property_signal("open", sidebar_state.settings_shown.signal())
                    .event(clone!(sidebar_state => move |_: events::Close| {
                        sidebar_state.settings_shown.set(false);
                    }))
                    .child(html!("jig-edit-sidebar-action-button", {
                        .property("slot", "anchor")
                        .property("kind", "settings")
                        .event(clone!(sidebar_state => move |_: events::Click| {
                            let mut settings_shown = sidebar_state.settings_shown.lock_mut();
                            *settings_shown = !*settings_shown;
                        }))
                    }))
                    .child(html!("div", {
                        .property("slot", "overlay")
                        .style("padding", "50px")
                        .style("border", "solid 1px gray")
                        .text("Settings")
                    }))
                }),
                html!("jig-edit-sidebar-action-button", {
                    .property("slot", "modules")
                    .property("kind", "modules")
                    .event(clone!(sidebar_state => move |_:events::Click| {
                        sidebar_state.route.set_neq(JigEditRoute::Landing);
                        let url:String = Route::Jig(JigRoute::Edit(sidebar_state.jig.id.clone(), JigEditRoute::Landing)).into();
                        dominator::routing::go_to_url(&url);
                    }))
                }),
                // old pencil input
                // html!("input-text-pencil", {
                //     .property("slot", "input")
                //     .property("placeholder", STR_SEARCH_PLACEHOLDER)
                //     .property_signal("value", sidebar_state.name.signal_cloned())
                //     .event(clone!(sidebar_state => move |e: events::CustomInput| {
                //         let value = e.value();
                //         sidebar_actions::update_display_name(sidebar_state.clone(), value);
                //     }))
                // }),
                html!("input-wrapper", {
                    .property("slot", "input")
                    .child(html!("input", {
                        .property("placeholder", STR_SEARCH_PLACEHOLDER)
                        .property_signal("value", sidebar_state.name.signal_cloned())
                        .event(clone!(sidebar_state => move |e: events::Input| {
                            let value = e.value().unwrap_or_default();
                            sidebar_actions::update_display_name(sidebar_state.clone(), value);
                        }))
                    }))
                    .child(html!("img-ui ", {
                        .property("slot", "icon")
                        .property("path", "core/inputs/pencil-blue-darker.svg")
                    }))
                }),
                html!("jig-edit-sidebar-preview-button", {
                    .property("slot", "preview")
                    .event(clone!(sidebar_state => move |_: events::Click| {
                        log::info!("Preview");
                    }))
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
