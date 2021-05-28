use super::state::*;
use std::rc::Rc;
use dominator::{html, clone, Dom};
use utils::prelude::*;
use futures_signals::signal::SignalExt;
use components::{
    image::search::dom::render as render_image_search
};

pub fn render(state: Rc<Step1>) -> Dom {
    html!("menu-tabs", {
        .children(&mut [
            render_tab(state.clone(), TabKind::Image),
            render_tab(state.clone(), TabKind::Color),
            render_tab(state.clone(), TabKind::Overlay),
            html!("module-sidebar-body", {
                .property("slot", "body")
                .child_signal(state.tab.signal_cloned().map(|tab| {
                    match tab {
                        Tab::Image(state) => {
                            Some(render_image_search(state.clone(), None))
                        },
                        _ => None,
                    }
                }))
            })
        ])
    })
}


fn render_tab(state: Rc<Step1>, tab_kind:TabKind) -> Dom {
    html!("menu-tab", {
        .property("slot", "tabs")
        .property_signal("active", state.tab.signal_ref(clone!(tab_kind => move |curr| {
            curr.kind() == tab_kind
        })))
        .child(html!("menu-tab-title", {
            .property("kind", tab_kind.as_str())
        }))
        .event(clone!(state, tab_kind => move |evt:events::Click| {
            state.tab.set(Tab::new(tab_kind));
        }))
    })
}
