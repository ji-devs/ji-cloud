use super::state::*;
use crate::base::sidebar::state::StickerPhase;
use components::{
    image::search::dom::render as render_image_search,
    tabs::{MenuTab, MenuTabKind},
    text_editor::dom::render_controls as render_text_editor,
};
use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use std::rc::Rc;

pub fn render_step_2(state: Rc<Step2>) -> Dom {
    state
        .sidebar
        .sticker_phase
        .set_neq(Some(StickerPhase::Scene));
    state.sidebar.trace_phase.set_neq(None);

    html!("menu-tabs", {
        .future(state.tab.signal_ref(|tab| tab.kind()).dedupe().for_each(clone!(state => move |kind| {
            state.sidebar.tab_kind.set(Some(kind));
            async move {}
        })))
        .children(&mut [
            render_tab(state.clone(), MenuTabKind::Text),
            render_tab(state.clone(), MenuTabKind::Image),
            html!("module-sidebar-body", {
                .property("slot", "body")
                .child_signal(state.tab.signal_cloned().map(clone!(state => move |tab| {
                    match tab {
                        Tab::StickerImage(state) => {
                            Some(render_image_search(state, None))
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
