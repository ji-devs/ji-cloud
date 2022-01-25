use super::state::*;
use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use std::rc::Rc;
use utils::prelude::*;

use crate::{
    image::search::dom::render as render_image_search,
    lists::{dual::dom::render as render_dual_list, single::dom::render as render_single_list},
    module::_groups::cards::edit::{state::*, strings},
    tabs::{MenuTab, MenuTabKind},
};

pub fn render<RawData: RawDataExt, E: ExtraExt>(state: Rc<Step1<RawData, E>>) -> Dom {
    html!("empty-fragment", {
        .style("display", "contents")
        .child_signal(state.base.is_empty_signal().map(clone!(state => move |is_empty| {
            Some(html!("menu-tabs", {
                .children(state.tabs.get().unwrap_ji().iter().enumerate().map(|(idx, tab)| {
                    let enabled = idx == 0 || (idx > 0 && !is_empty);
                    render_tab(state.clone(), tab.kind(), idx, enabled)
                }))
                .child(html!("module-sidebar-body", {
                        .property("slot", "body")
                        .child_signal(state.tab_index.signal_cloned().map(clone!(state, is_empty => move |current_tab_idx| {
                            let tab = match current_tab_idx {
                                Some(current_tab_idx) => match state.tabs.get() {
                                    Some(tabs) => tabs.get(current_tab_idx),
                                    None => None,
                                },
                                None => None,
                            };
                            match tab {
                                Some(Tab::Single(single)) => {
                                    if !is_empty {
                                        Some(render_non_empty(state.clone()))
                                    } else {
                                        Some(render_single_list(single.clone()))
                                    }
                                },
                                Some(Tab::Dual(dual)) => {
                                    if !is_empty {
                                        Some(render_non_empty(state.clone()))
                                    } else {
                                        Some(render_dual_list(dual.clone()))
                                    }
                                }
                                Some(Tab::Image(image)) => {
                                    Some(render_image_search(image.clone(), None))
                                },
                                _ => None,
                            }
                        })))
                    })
                )
            }))
        })))
    })
}

fn render_non_empty<RawData: RawDataExt, E: ExtraExt>(state: Rc<Step1<RawData, E>>) -> Dom {
    html!("sidebar-empty", {
        .child(
            html!("button-rect", {
                .property("slot", "clear")
                .property("kind", "text")
                .property("color", "blue")
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
    tab_kind: MenuTabKind,
    idx: usize,
    enabled: bool,
) -> Dom {
    MenuTab::render(
        MenuTab::new(
            tab_kind,
            false,
            enabled,
            clone!(state => move || state.tab_index.signal_ref(move |current_tab_idx| {
                current_tab_idx.as_ref().map_or(false, |current_tab_idx| *current_tab_idx == idx)
            })),
            clone!(state => move || {
                state.tab_index.set_neq(Some(idx))
            }),
        ),
        Some("tabs"),
    )
}
