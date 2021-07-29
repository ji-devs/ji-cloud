use super::state::*;
use std::rc::Rc;
use dominator::{html, clone, Dom};
use utils::prelude::*;
use futures_signals::signal::SignalExt;
use crate::{
    image::search::dom::render as render_image_search,
    color_select::dom::render as render_color_picker,
    theme_selector::dom::render_cards as render_theme_selector,
    module::_groups::cards::edit::state::*
};

pub fn render<RawData: RawDataExt, E: ExtraExt>(state: Rc<Step2<RawData, E>>) -> Dom {
    html!("menu-tabs", {
        .children(&mut [
            render_tab(state.clone(), TabKind::Theme),
            render_tab(state.clone(), TabKind::Image),
            render_tab(state.clone(), TabKind::Color),
            html!("module-sidebar-body", {
                .property("slot", "body")
                .child_signal(state.tab.signal_cloned().map(|tab| {
                    match tab {
                        Tab::Theme(state) => {
                            Some(render_theme_selector(state.clone(), None))
                        },
                        Tab::Image(state) => {
                            Some(render_image_search(state.clone(), None))
                        },
                        Tab::Color(state) => {
                            Some(render_color_picker(state.clone(), None))
                        },
                    }
                }))
            })
        ])
    })
}


fn render_tab<RawData: RawDataExt, E: ExtraExt>(state: Rc<Step2<RawData, E>>, tab_kind:TabKind) -> Dom {
    html!("menu-tab-with-title", {
        .property("slot", "tabs")
        .property_signal("active", state.tab.signal_ref(clone!(tab_kind => move |curr| {
            curr.kind() == tab_kind
        })))
        .property("kind", tab_kind.as_str())
        .event(clone!(state, tab_kind => move |evt:events::Click| {
            state.tab.set(Tab::new(state.base.clone(), tab_kind));
        }))
    })
}
