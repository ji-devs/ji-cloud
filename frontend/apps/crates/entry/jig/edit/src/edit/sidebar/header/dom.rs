use dominator::{html, Dom};
use shared::domain::jig::Jig;
use std::rc::Rc;
use super::super::state::State as SidebarState;
use utils::prelude::*;

pub struct HeaderDom {
}

impl HeaderDom {
    pub fn render(sidebar_state: Rc<SidebarState>) -> Dom {
        html!("jig-edit-sidebar-header", {
            .property("slot", "header")
            .children(&mut [
                html!("button-icon", {
                    .property("slot", "close")
                    .property("icon", "x")
                }),
                html!("button-text", {
                    .property("slot", "gallery")
                    .property("color", "blue")
                    .property("weight", "medium")
                    .text(crate::strings::STR_MY_JIGS)
                    .event(|evt:events::Click| {
                        let url:String = Route::Jig(JigRoute::Gallery).into();
                        dominator::routing::go_to_url(&url);
                    })
                }),
                html!("input-text-pencil", {
                    .property("slot", "input")
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
