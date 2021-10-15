use super::{state::*, video};
use components::{
    image::search::dom::render as render_image_search,
    tabs::{MenuTab, MenuTabKind},
    text_editor::dom::render_controls as render_text_editor,
};
use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use std::rc::Rc;

pub fn render(state: Rc<Step2>) -> Dom {
    html!("menu-tabs", {
        .future(state.tab.signal_ref(|tab| tab.as_index()).dedupe().for_each(clone!(state => move |index| {
            state.sidebar.tab_index.set(Some(index));
            async move {}
        })))
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
                            Some(render_text_editor(state.sidebar.base.text_editor.clone()))
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

fn render_tab(state: Rc<Step2>, tab_kind: MenuTabKind) -> Dom {
    MenuTab::render(
        MenuTab::new(
            tab_kind,
            false,
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
