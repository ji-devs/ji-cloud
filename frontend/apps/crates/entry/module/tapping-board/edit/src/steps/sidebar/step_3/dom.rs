use super::state::*;
use std::rc::Rc;
use dominator::{html, clone, Dom};
use utils::prelude::*;
use futures_signals::signal::SignalExt;

pub fn render(state: Rc<Step3>) -> Dom {
    html!("menu-tabs", {
        .children(&mut [
            render_tab(state.clone(), TabKind::Text),
            render_tab(state.clone(), TabKind::Audio),
            html!("module-sidebar-body", {
                .property("slot", "body")
                .child_signal(state.tab.signal_cloned().map(|tab| {
                    match tab {
                        Tab::Text(state) => {
                            None
                            //Some(render_image_search(state.clone(), None))
                        },
                        _ => None,
                    }
                }))
            })
        ])
    })
}


fn render_tab(state: Rc<Step3>, tab_kind:TabKind) -> Dom {
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
