use super::state::*;
use components::{
    instructions::editor::dom::render as render_instructions,
    tabs::{MenuTab, MenuTabKind},
};
use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use std::rc::Rc;

pub fn render(state: Rc<Step4>) -> Dom {
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
            render_tab(state.clone(), MenuTabKind::PlaySettings),
            render_tab(state.clone(), MenuTabKind::Instructions),
            render_tab(state.clone(), MenuTabKind::Feedback),
            html!("module-sidebar-body", {
                .property("slot", "body")
                .style("overflow", "inherit") // Inherit overflow otherwise the Hebrew controls will be hidden
                .child_signal(state.tab.signal_cloned().map(|tab| {
                    match tab {
                        Tab::Settings(state) => {
                            Some(super::play_settings::dom::render(state))
                        },
                        Tab::Instructions(state) => {
                            Some(render_instructions(state))
                        },
                        Tab::Feedback(state) => {
                            Some(render_instructions(state))
                        }
                    }
                }))
            })
        ])
    })
}

fn render_tab(state: Rc<Step4>, tab_kind: MenuTabKind) -> Dom {
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
