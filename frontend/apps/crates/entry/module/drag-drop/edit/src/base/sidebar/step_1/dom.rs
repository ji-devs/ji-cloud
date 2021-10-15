use super::state::*;
use components::{
    color_select::dom::render as render_color_picker,
    image::search::dom::render as render_image_search,
    tabs::{MenuTab, MenuTabKind},
    text_editor::dom::render_controls as render_text_editor,
};
use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use std::rc::Rc;
use utils::prelude::*;

pub fn render_step_1(state: Rc<Step1>) -> Dom {
    html!("menu-tabs", {
        .future(state.tab.signal_ref(|tab| tab.as_index()).dedupe().for_each(clone!(state => move |index| {
            state.sidebar.tab_index.set(Some(index));
            async move {}
        })))
        .children(&mut [
            render_tab(state.clone(), MenuTabKind::BackgroundImageFull),
            render_tab(state.clone(), MenuTabKind::BackgroundColor),
            render_tab(state.clone(), MenuTabKind::Overlay),
            render_tab(state.clone(), MenuTabKind::Image),
            render_tab(state.clone(), MenuTabKind::Text),
            html!("module-sidebar-body", {
                .property("slot", "body")
                .child_signal(state.tab.signal_cloned().map(clone!(state => move |tab| {
                    match tab {
                        Tab::BgImage(state) => {
                            Some(render_image_search(state.clone(), None))
                        },
                        Tab::BgColor(state) => {
                            Some(render_color_picker(state.clone(), None))
                        },
                        Tab::BgOverlay(state) => {
                            Some(render_image_search(state.clone(), None))
                        },
                        Tab::StickerImage(state) => {
                            Some(render_image_search(state.clone(), None))
                        },
                        Tab::StickerText => {
                            Some(render_text_editor(state.sidebar.base.text_editor.clone()))
                        },
                    }
                })))
            })
        ])
    })
}

fn render_tab(state: Rc<Step1>, tab_kind: MenuTabKind) -> Dom {
    MenuTab::render(
        MenuTab::new(
            tab_kind,
            true,
            clone!(state => move || state.tab.signal_ref(clone!(tab_kind => move |curr| {
                curr.kind() == tab_kind
            }))),
            clone!(state, tab_kind => move || {
                state.tab.set(Tab::new(state.sidebar.base.clone(), tab_kind));
            }),
        ),
        Some("tabs"),
    )
}
