use super::state::*;
use dominator::{clone, html, Dom};
use futures_signals::signal::Mutable;
use futures_signals::signal::SignalExt;
use std::rc::Rc;
use utils::prelude::*;

use crate::{
    tabs::{MenuTab, MenuTabKind},
    image::search::dom::render as render_image_search,
    lists::{dual::dom::render as render_dual_list, single::dom::render as render_single_list},
    module::_groups::cards::edit::{state::*, strings},
};

pub fn render<RawData: RawDataExt, E: ExtraExt>(state: Rc<Step1<RawData, E>>) -> Dom {
    html!("empty-fragment", {
        .style("display", "contents")
        .child_signal(state.base.is_empty_signal().map(clone!(state => move |is_empty| {
            Some(match &*state.widget {
                Widget::Single(single) => {

                    html!("module-sidebar-body", {
                        .property("slot", "body")
                        .child(
                            if !is_empty {
                                render_non_empty(state.clone())
                            } else {
                                render_single_list(single.clone())
                            }
                        )
                    })
                },
                Widget::Dual(dual) => {
                    html!("module-sidebar-body", {
                        .property("slot", "body")
                        .child(
                            if !is_empty {
                                render_non_empty(state.clone())
                            } else {
                                render_dual_list(dual.clone())
                            }
                        )
                    })
                },
                Widget::Tabs(tab) => {
                    html!("menu-tabs", {
                        .future(tab.signal_ref(|tab| tab.as_index()).dedupe().for_each(clone!(state => move |index| {
                            state.tab_index.set(Some(index));
                            async move {}
                        })))
                        .children(&mut [
                            render_tab(state.clone(), tab.clone(), MenuTabKind::Text, true),
                            render_tab(state.clone(), tab.clone(), MenuTabKind::Image, !is_empty),
                            html!("module-sidebar-body", {
                                .property("slot", "body")
                                .child_signal(tab.signal_cloned().map(clone!(state, is_empty => move |tab| {
                                    match tab {
                                        Tab::Text(single) => {
                                            Some(if !is_empty {
                                                render_non_empty(state.clone())
                                            } else {
                                                render_single_list(single.clone())
                                            })
                                        },
                                        Tab::Image(image) => {
                                            Some(render_image_search(image.clone(), None))
                                        },
                                    }
                                })))
                            })
                        ])
                    })
                }
            })
        })))
    })
}

fn render_non_empty<RawData: RawDataExt, E: ExtraExt>(state: Rc<Step1<RawData, E>>) -> Dom {
    html!("sidebar-empty", {
        .child(
            html!("button-rect", {
                .property("slot", "clear")
                .property("kind", "text")
                .text(strings::STR_CREATE_NEW_LIST)
                .event(clone!(state => move |_evt:events::Click| {
                    state.base.clear_all();
                }))
            })
        )
    })
}

fn render_tab<RawData: RawDataExt, E: ExtraExt>(
    state: Rc<Step1<RawData, E>>,
    tab: Mutable<Tab>,
    tab_kind: MenuTabKind,
    enabled: bool
) -> Dom {
    MenuTab::render(
        MenuTab::new(
            tab_kind,
            false,
            clone!(tab => move || tab.signal_ref(clone!(tab_kind => move |curr| {
                curr.kind() == tab_kind
            }))),
            clone!(state, tab_kind => move || {
                tab.set(Tab::new(state.base.clone(), tab_kind));
            })
        ),
        Some("tabs")
    )
}