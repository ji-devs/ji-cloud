use super::state::*;
use components::{
    image::search::dom::render as render_image_search,
    tabs::{MenuTab, MenuTabKind},
};
use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use std::rc::Rc;

pub fn render(state: Rc<Step2>) -> Dom {
    html!("menu-tabs", {
        .future(state.tab.signal_ref(|tab| tab.kind()).dedupe().for_each(clone!(state => move |kind| {
            state.sidebar.tab_kind.set(Some(kind));
            state.sidebar.base.continue_next_fn.set(Some(Rc::new(clone!(state => move || {
                if let Some(kind) = next_kind(&kind) {
                        state.tab.set(Tab::new(state.sidebar.base.clone(), kind));
                    true
                } else {
                    false
                }
            }))));
            async move {}
        })))
        .children(&mut [
            render_tab(state.clone(), MenuTabKind::Text),
            render_tab(state.clone(), MenuTabKind::Image),
            html!("module-sidebar-body", {
                .property("slot", "body")
                .child_signal(state.tab.signal_cloned().map(clone!(state => move |tab| {
                    match tab {
                        Tab::Text => {
                            Some(state.sidebar.base.text_editor.render_controls())
                        },
                        Tab::Image(state) => {
                            Some(render_image_search(state, None))
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