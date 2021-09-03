use super::{video, state::*};
use components::{
    tabs::{MenuTab, MenuTabKind},
    image::search::dom::render as render_image_search,
    text_editor::dom::render_controls as render_text_editor,
};
use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use std::rc::Rc;
use utils::prelude::*;

pub fn render(state: Rc<Step2>) -> Dom {
    html!("menu-tabs", {
        .children(&mut [
            render_tab(state.clone(), MenuTabKind::Video),
            render_tab(state.clone(), MenuTabKind::Text),
            render_tab(state.clone(), MenuTabKind::Image),
            html!("module-sidebar-body", {
                .property("slot", "body")
                .child_signal(state.tab.signal_cloned().map(clone!(state => move |tab| {
                    match tab {
                        Tab::Video => {
                            Some(video::render(Rc::clone(&state)))
                        },
                        Tab::Text => {
                            Some(render_text_editor(state.base.text_editor.clone()))
                        },
                        Tab::Image(state) => {
                            Some(render_image_search(state.clone(), None))
                        },
                    }
                })))
            })
        ])
    })
}

fn render_tab(state: Rc<Step2>, tab_kind:MenuTabKind) -> Dom {
    MenuTab::render(
        MenuTab::new(
            tab_kind,
            false,
            clone!(state => move || state.tab.signal_ref(clone!(tab_kind => move |curr| {
                curr.kind() == tab_kind
            }))),
            clone!(state, tab_kind => move || {
                state.tab.set(Tab::new(state.base.clone(), tab_kind));
            })
        ),
        Some("tabs")
    )
}