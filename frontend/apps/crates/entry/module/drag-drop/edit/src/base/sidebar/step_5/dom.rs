use super::state::*;
use std::rc::Rc;
use dominator::{html, clone, Dom};
use utils::prelude::*;
use futures_signals::signal::{Signal, SignalExt};
use components::{
    instructions::editor::dom::render as render_instructions,
};

pub fn render_step_5(state: Rc<Step5>) -> Dom {
    html!("menu-tabs", {
        .children(&mut [
            render_tab(state.clone(), TabKind::Settings),
            render_tab(state.clone(), TabKind::Instructions),
            render_tab(state.clone(), TabKind::Feedback),
            html!("module-sidebar-body", {
                .property("slot", "body")
                .child_signal(state.tab.signal_cloned().map(clone!(state => move |tab| {
                    match tab {

                        Tab::Settings(state) => {
                            Some(super::play_settings::dom::render(state.clone()))
                        },
                        Tab::Instructions(state) => {
                            Some(render_instructions(state.clone()))
                        },
                        Tab::Feedback(state) => {
                            Some(render_instructions(state.clone()))
                        },
                    }
                })))
            })
        ])
    })
}


fn render_tab(state: Rc<Step5>, tab_kind:TabKind) -> Dom {
    html!("menu-tab-with-title", {
        .property("slot", "tabs")
        .property("kind", tab_kind.as_str())
        .property_signal("active", state.tab.signal_ref(clone!(tab_kind => move |curr| {
            curr.kind() == tab_kind
        })))
        .event(clone!(state, tab_kind => move |evt:events::Click| {
            state.tab.set(Tab::new(state.base.clone(), tab_kind));
        }))
    })
}

