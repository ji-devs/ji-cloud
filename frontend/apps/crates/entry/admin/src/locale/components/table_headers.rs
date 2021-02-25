use crate::locale::state::{Column, SortKind, State};
use crate::locale::state::{ItemKind, Section};
use dominator::{clone, html, with_node, Dom};
use futures_signals::{signal::SignalExt, signal_vec::SignalVecExt};
use std::collections::BTreeMap;
use std::rc::Rc;
use utils::events;
use web_sys::HtmlSelectElement;

pub struct TableHeaderDom {}

impl TableHeaderDom {
    pub fn render(state: Rc<State>) -> Dom {
        html!("locale-row", {
            .property("slot", "rows")
            .children_signal_vec(state.visible_columns.signal_vec_cloned()
                .map(clone!(state => move |column| {
                    match column {
                        Column::ID => {
                            html!("locale-cell-header", {
                                .property("label", Column::ID.to_string())
                            })
                        }
                        Column::Section => {
                            html!("locale-cell-header", {
                                .property("label", Column::Section.to_string())
                                .children(&mut [
                                    html!("select" => HtmlSelectElement, {
                                        .with_node!(elem => {
                                            .property("slot", "actions")
                                            .attribute("multiple", "")
                                            .style("width", "100px")
                                            .children_signal_vec(state.section_options.signal_cloned()
                                                .map(|section_options: BTreeMap<Section, bool>| {
                                                    section_options.iter().map(|(section_option, visible): (&Section, &bool)| {
                                                        html!("option", {
                                                            .property("value", section_option)
                                                            .property("selected", *visible)
                                                            .text(&section_option)
                                                        })
                                                    }).collect()
                                                })
                                                .to_signal_vec())
                                            .event(clone!(state => move |_event: events::Change| {
                                                let options = elem.options();
                                                let mut section_options = state.section_options.lock_mut();

                                                State::filter_change(&options, &mut section_options);
                                            }))
                                        })
                                    }),
                                    html!("locale-sort-button", {
                                        .property("slot", "actions")
                                        .property_signal("sorted", state.sort.signal_ref(|sort| sort.column == SortKind::Section))
                                        .event(clone!(state => move |_event: events::Click| {
                                            state.sort_clicked(SortKind::Section);
                                        }))
                                    }),
                                ])
                            })
                        }
                        Column::ItemKind => {
                            html!("locale-cell-header", {
                                .property("label", Column::ItemKind.to_string())
                                .children(&mut [
                                    html!("select" => HtmlSelectElement, {
                                        .with_node!(elem => {
                                            .property("slot", "actions")
                                            .attribute("multiple", "")
                                            .style("width", "100px")
                                            .children_signal_vec(state.item_kind_options.signal_cloned()
                                                .map(|item_kind_options: BTreeMap<ItemKind, bool>| {
                                                    item_kind_options.iter().map(|(item_kind_option, visible): (&ItemKind, &bool)| {
                                                        html!("option", {
                                                            .property("value", item_kind_option)
                                                            .property("selected", *visible)
                                                            .text(&item_kind_option)
                                                        })
                                                    }).collect()
                                                })
                                                .to_signal_vec())
                                            .event(clone!(state => move |_event: events::Change| {
                                                let options = elem.options();
                                                let mut item_kind_options = state.item_kind_options.lock_mut();

                                                State::filter_change(&options, &mut item_kind_options);
                                            }))
                                        })
                                    }),
                                    html!("locale-sort-button", {
                                        .property("slot", "actions")
                                        .property_signal("sorted", state.sort.signal_ref(|sort| sort.column == SortKind::ItemKind))
                                        .event(clone!(state => move |_event: events::Click| {
                                            state.sort_clicked(SortKind::ItemKind);
                                        }))
                                    }),
                                ])
                            })
                        }
                        Column::English => {
                            html!("locale-cell-header", {
                                .property("label", Column::English.to_string())
                                .child(html!("locale-sort-button", {
                                    .property("slot", "actions")
                                    .property_signal("sorted", state.sort.signal_ref(|sort| sort.column == SortKind::English))
                                    .event(clone!(state => move |_event: events::Click| {
                                        state.sort_clicked(SortKind::English);
                                    }))
                                }))
                            })
                        }
                        Column::Hebrew => {
                            html!("locale-cell-header", {
                                .property("label", Column::Hebrew.to_string())
                                .child(html!("locale-sort-button", {
                                    .property("slot", "actions")
                                    .property_signal("sorted", state.sort.signal_ref(|sort| sort.column == SortKind::Hebrew))
                                    .event(clone!(state => move |_event: events::Click| {
                                        state.sort_clicked(SortKind::Hebrew);
                                    }))
                                }))
                            })
                        }
                        Column::Status => {
                            html!("locale-cell-header", {
                                .property("label", Column::Status.to_string())
                                .children(&mut [
                                    html!("select" => HtmlSelectElement, {
                                        .with_node!(elem => {
                                            .property("slot", "actions")
                                            .attribute("multiple", "")
                                            .style("width", "100px")
                                            .children(

                                                state.status_options.lock_ref().iter().map(|(o, selected)| {
                                                    html!("option", {
                                                        .property("text", o.to_string())
                                                        .property("value", o.to_string())
                                                        .property("selected", *selected)
                                                    })
                                                })
                                            )
                                            .event(clone!(state => move |_event: events::Change| {
                                                let options = elem.options();
                                                let mut status_options = state.status_options.lock_mut();

                                                State::filter_change(&options, &mut status_options);
                                            }))
                                        })
                                    }),
                                    html!("locale-sort-button", {
                                        .property("slot", "actions")
                                        .property_signal("sorted", state.sort.signal_ref(|sort| sort.column == SortKind::Status))
                                        .event(clone!(state => move |_event: events::Click| {
                                            state.sort_clicked(SortKind::Status);
                                        }))
                                    }),
                                ])
                            })
                        }
                        Column::ZeplinReference => {
                            html!("locale-cell-header", {
                                .property("label", Column::ZeplinReference.to_string())
                            })
                        }
                        Column::Comments => {
                            html!("locale-cell-header", {
                                .property("label", Column::Comments.to_string())
                                .child(html!("locale-sort-button", {
                                    .property("slot", "actions")
                                    .property_signal("sorted", state.sort.signal_ref(|sort| sort.column == SortKind::Comments))
                                    .event(clone!(state => move |_event: events::Click| {
                                        state.sort_clicked(SortKind::Comments);
                                    }))
                                }))
                            })
                        }
                        Column::App => {
                            html!("locale-cell-header", {
                                .property("label", Column::App.to_string())
                                .property("adminOnly", true)
                            })
                        }
                        Column::Element => {
                            html!("locale-cell-header", {
                                .property("label", Column::Element.to_string())
                                .property("adminOnly", true)
                            })
                        }
                        Column::Mock => {
                            html!("locale-cell-header", {
                                .property("label", Column::Mock.to_string())
                                .property("adminOnly", true)
                            })
                        }
                        Column::Actions => {
                            html!("locale-cell-header", {
                            })
                        }
                    }
                }))
            )
        })
    }
}
