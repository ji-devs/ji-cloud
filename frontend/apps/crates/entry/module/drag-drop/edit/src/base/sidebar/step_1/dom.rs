use super::state::*;
use std::rc::Rc;
use dominator::{html, clone, Dom};
use utils::prelude::*;
use futures_signals::signal::SignalExt;
use components::{
    image::search::dom::render as render_image_search,
    color_select::dom::render as render_color_picker,
    text_editor::dom::render_controls as render_text_editor,
};

pub fn render(state: Rc<Step1>) -> Dom {
    html!("menu-tabs", {
        .children(&mut [
            render_tab(state.clone(), TabKind::BgImage),
            render_tab(state.clone(), TabKind::BgColor),
            render_tab(state.clone(), TabKind::BgOverlay),
            render_tab(state.clone(), TabKind::StickerImage),
            render_tab(state.clone(), TabKind::StickerText),
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
                            Some(render_text_editor(state.base.text_editor.clone()))
                        },
                    }
                })))
            })
        ])
    })
}


fn render_tab(state: Rc<Step1>, tab_kind:TabKind) -> Dom {
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
