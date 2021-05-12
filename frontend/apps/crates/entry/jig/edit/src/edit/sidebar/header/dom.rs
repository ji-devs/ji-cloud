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
            .property_signal("isModulePage", sidebar_state.module_id.signal_cloned().map(|module_id| module_id.is_none()))
            .children(&mut [
                html!("img-ui", {
                    .property("slot", "close")
                    .property("path", "entry/jig/collapse.svg")
                    .event(clone!(sidebar_state => move |_: events::Click| {
                        let mut collapsed = sidebar_state.collapsed.lock_mut();
                        *collapsed = !*collapsed;
                    }))
                }),
                html!("button-text", {
                    .property("slot", "gallery")
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
                    .property("backdropColor", "transparent")
                    .property_signal("open", sidebar_state.settings_shown.signal())
                    .event(clone!(sidebar_state => move |_: events::Close| {
                        sidebar_state.settings_shown.set(false);
                    }))
                    .child(html!("img-ui", {
                        .property("slot", "anchor")
                        .property("path", "entry/jig/settings.svg")
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
                html!("img-ui", {
                    .property("slot", "modules")
                    .property("path", "entry/jig/modules.svg")
                    .event(clone!(sidebar_state => move |_:events::Click| {
                        sidebar_state.module_id.set_neq(None);
                        let url:String = Route::Jig(JigRoute::Edit(sidebar_state.jig.id.clone(), None)).into();
                        dominator::routing::go_to_url(&url);
                    }))
                }),
                html!("input-text-pencil", {
                    .property("slot", "input")
                    .property("placeholder", STR_SEARCH_PLACEHOLDER)
                    .property_signal("value", sidebar_state.name.signal_cloned().map(|value| {
                        value.unwrap_or(String::new())
                    }))
                    .event(clone!(sidebar_state => move |e: events::CustomInput| {
                        let value = e.value();
                        sidebar_actions::update_display_name(sidebar_state.clone(), value);
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
