use super::state::*;
use std::rc::Rc;
use dominator::{html, clone, Dom, with_node};
use utils::prelude::*;
use futures_signals::signal::SignalExt;
use futures_signals::signal::Mutable;

use crate::{
    image::search::dom::render as render_image_search,
    lists::{
        single::dom::render as render_single_list,
        dual::dom::render as render_dual_list,
    },
    module::_groups::cards::edit::{
        state::*,
        strings
    }
};

pub fn render<RawData: RawDataExt, E: ExtraExt>(state: Rc<Step1<RawData, E>>) -> Dom {
    html!("empty-fragment", {
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
                        .children(&mut [
                            render_tab(state.clone(), tab.clone(), TabKind::Text),
                            render_tab(state.clone(), tab.clone(), TabKind::Image),
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
                .event(clone!(state => move |evt:events::Click| {
                    state.base.clear_all();
                }))
            })
        )
    })
}

fn render_tab<RawData: RawDataExt, E: ExtraExt>(state: Rc<Step1<RawData, E>>, tab: Mutable<Tab>, tab_kind:TabKind) -> Dom {
    html!("menu-tab-with-title", {
        .property("slot", "tabs")
        .property_signal("active", tab.signal_ref(clone!(tab_kind => move |curr| {
            curr.kind() == tab_kind
        })))
        .property("kind", tab_kind.as_str())
        .event(clone!(state, tab_kind, tab => move |evt:events::Click| {
            tab.set(Tab::new(state.base.clone(), tab_kind));
        }))
    })
}

