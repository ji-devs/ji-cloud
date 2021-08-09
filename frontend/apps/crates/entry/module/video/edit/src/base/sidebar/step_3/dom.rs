use super::state::*;
use components::instructions::editor::dom::render as render_instructions;
use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use std::rc::Rc;
use utils::prelude::*;

pub fn render(state: Rc<Step3>) -> Dom {
    html!("menu-tabs", {
        .children(&mut [
            render_tab(state.clone(), TabKind::Settings),
            render_tab(state.clone(), TabKind::Instructions),
            html!("module-sidebar-body", {
                .property("slot", "body")
                .child_signal(state.tab.signal_cloned().map(move |tab| {
                    match tab {
                        Tab::Settings(state) => {
                            Some(super::play_settings::dom::render(state.clone()))
                        },
                        Tab::Instructions(state) => {
                            Some(render_instructions(state.clone()))
                        },
                    }
                }))
            })
        ])
    })
}

fn render_tab(state: Rc<Step3>, tab_kind: TabKind) -> Dom {
    html!("menu-tab-with-title", {
        .property("slot", "tabs")
        .property("kind", tab_kind.as_str())
        .property_signal("active", state.tab.signal_ref(clone!(tab_kind => move |curr| {
            curr.kind() == tab_kind
        })))
        .event(clone!(state, tab_kind => move |_:events::Click| {
            state.tab.set(Tab::new(state.base.clone(), tab_kind));
        }))
    })
}
