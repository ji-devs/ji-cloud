use super::state::*;
use crate::{
    color_select::dom::render as render_color_picker,
    image::search::dom::render as render_image_search,
    module::_groups::cards::edit::state::*,
    tabs::{MenuTab, MenuTabKind},
    theme_selector::dom::render_cards as render_theme_selector,
};
use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use std::rc::Rc;

pub fn render<RawData: RawDataExt, E: ExtraExt>(state: Rc<Step2<RawData, E>>) -> Dom {
    html!("menu-tabs", {
        .future(state.tab.signal_ref(|tab| tab.as_index()).dedupe().for_each(clone!(state => move |index| {
            state.tab_index.set(Some(index));
            async move {}
        })))
        .children(&mut [
            render_tab(state.clone(), MenuTabKind::Theme),
            render_tab(state.clone(), MenuTabKind::Image),
            render_tab(state.clone(), MenuTabKind::Color),
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

fn render_tab<RawData: RawDataExt, E: ExtraExt>(
    state: Rc<Step2<RawData, E>>,
    tab_kind: MenuTabKind,
) -> Dom {
    MenuTab::render(
        MenuTab::new(
            tab_kind,
            false,
            clone!(state => move || state.tab.signal_ref(clone!(tab_kind => move |curr| {
                curr.kind() == tab_kind
            }))),
            clone!(state, tab_kind => move || {
                state.tab.set(Tab::new(state.base.clone(), tab_kind));
            }),
        ),
        Some("tabs"),
    )
}
