use crate::locale::actions::AsStringExt;
use crate::locale::state::Section;
use crate::locale::state::{Column, SortKind, State};
use dominator::{clone, html, with_node, Dom};
use futures_signals::{signal::SignalExt, signal_vec::SignalVecExt};
use shared::domain::locale::ItemKind;
use std::collections::BTreeMap;
use std::rc::Rc;
use utils::events;
use utils::unwrap::UnwrapJiExt;
use uuid::Uuid;
use wasm_bindgen::JsCast;
use web_sys::{HtmlOptionElement, HtmlSelectElement};

pub struct TableHeaderDom {}

impl TableHeaderDom {
    pub fn render(state: Rc<State>) -> Dom {
        html!("locale-row", {
            .prop("slot", "rows")
            .children_signal_vec(state.visible_columns.signal_vec_cloned()
                .map(clone!(state => move |column| {
                    match column {
                        Column::ID => {
                            html!("locale-cell-header", {
                                .prop("label", Column::ID.to_string())
                            })
                        }
                        Column::Section => {
                            html!("locale-cell-header", {
                                .prop("label", Column::Section.to_string())
                                .children(&mut [
                                    html!("select" => HtmlSelectElement, {
                                        .with_node!(elem => {
                                            .prop("slot", "actions")
                                            .attr("multiple", "")
                                            .style("width", "100px")
                                            .children_signal_vec(state.section_options.signal_cloned()
                                                .map(|section_options: BTreeMap<Section, bool>| {
                                                    section_options.iter().map(|(section_option, visible): (&Section, &bool)| {
                                                        html!("option", {
                                                            .prop("value", section_option)
                                                            .prop("selected", *visible)
                                                            .text(section_option)
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
                                        .prop("slot", "actions")
                                        .prop_signal("sorted", state.sort.signal_ref(|sort| sort.column == SortKind::Section))
                                        .event(clone!(state => move |_event: events::Click| {
                                            state.sort_clicked(SortKind::Section);
                                        }))
                                    }),
                                ])
                            })
                        }
                        Column::ItemKind => {
                            html!("locale-cell-header", {
                                .prop("label", Column::ItemKind.to_string())
                                .children(&mut [
                                    html!("select" => HtmlSelectElement, {
                                        .with_node!(elem => {
                                            .prop("slot", "actions")
                                            .attr("multiple", "")
                                            .style("width", "100px")
                                            .child(html!("option", {
                                                .prop("value", "")
                                                .prop("selected", *state.item_kind_filter.lock_ref().get(&None).unwrap_ji())
                                            }))
                                            .children(state
                                                .item_kind_options
                                                .iter()
                                                .map(|item_kind: &ItemKind| {
                                                    html!("option", {
                                                        .prop("value", &item_kind.id.to_string())
                                                        .prop("selected", *state.item_kind_filter.lock_ref().get(&Some(item_kind.id)).unwrap_ji())
                                                        .text(&item_kind.name)
                                                    })
                                                })
                                            )
                                            .event(clone!(state => move |_event: events::Change| {
                                                let options = elem.options();
                                                let mut item_kind_filter = state.item_kind_filter.lock_mut();
                                                for i in 0..options.length() {

                                                    let option: HtmlOptionElement = options.get_with_index(i).unwrap_ji().dyn_into::<HtmlOptionElement>().unwrap_ji();
                                                    let value = option.value();
                                                    let value = match Uuid::parse_str(&value) {
                                                        Ok(uuid) => Some(uuid),
                                                        Err(_) => None,
                                                    };
                                                    item_kind_filter.insert(value, option.selected());

                                                }
                                            }))
                                        })
                                    }),
                                    html!("locale-sort-button", {
                                        .prop("slot", "actions")
                                        .prop_signal("sorted", state.sort.signal_ref(|sort| sort.column == SortKind::ItemKind))
                                        .event(clone!(state => move |_event: events::Click| {
                                            state.sort_clicked(SortKind::ItemKind);
                                        }))
                                    }),
                                ])
                            })
                        }
                        Column::English => {
                            html!("locale-cell-header", {
                                .prop("label", Column::English.to_string())
                                .child(html!("locale-sort-button", {
                                    .prop("slot", "actions")
                                    .prop_signal("sorted", state.sort.signal_ref(|sort| sort.column == SortKind::English))
                                    .event(clone!(state => move |_event: events::Click| {
                                        state.sort_clicked(SortKind::English);
                                    }))
                                }))
                            })
                        }
                        Column::Hebrew => {
                            html!("locale-cell-header", {
                                .prop("label", Column::Hebrew.to_string())
                                .child(html!("locale-sort-button", {
                                    .prop("slot", "actions")
                                    .prop_signal("sorted", state.sort.signal_ref(|sort| sort.column == SortKind::Hebrew))
                                    .event(clone!(state => move |_event: events::Click| {
                                        state.sort_clicked(SortKind::Hebrew);
                                    }))
                                }))
                            })
                        }
                        Column::Status => {
                            html!("locale-cell-header", {
                                .prop("label", Column::Status.to_string())
                                .children(&mut [
                                    html!("select" => HtmlSelectElement, {
                                        .with_node!(elem => {
                                            .prop("slot", "actions")
                                            .attr("multiple", "")
                                            .style("width", "100px")
                                            .children(

                                                state.status_options.lock_ref().iter().map(|(o, selected)| {
                                                    html!("option", {
                                                        .prop("text", &o.to_string())
                                                        .prop("value", &o.to_string())
                                                        .prop("selected", *selected)
                                                    })
                                                })
                                            )
                                            .event(clone!(state => move |_event: events::Change| {
                                                let options = elem.options();
                                                let mut status_options = state.status_options.lock_mut();

                                                State::filter_change_str_ext(&options, &mut status_options);
                                            }))
                                        })
                                    }),
                                    html!("locale-sort-button", {
                                        .prop("slot", "actions")
                                        .prop_signal("sorted", state.sort.signal_ref(|sort| sort.column == SortKind::Status))
                                        .event(clone!(state => move |_event: events::Click| {
                                            state.sort_clicked(SortKind::Status);
                                        }))
                                    }),
                                ])
                            })
                        }
                        Column::ZeplinReference => {
                            html!("locale-cell-header", {
                                .prop("label", Column::ZeplinReference.to_string())
                            })
                        }
                        Column::Comments => {
                            html!("locale-cell-header", {
                                .prop("label", Column::Comments.to_string())
                                .child(html!("locale-sort-button", {
                                    .prop("slot", "actions")
                                    .prop_signal("sorted", state.sort.signal_ref(|sort| sort.column == SortKind::Comments))
                                    .event(clone!(state => move |_event: events::Click| {
                                        state.sort_clicked(SortKind::Comments);
                                    }))
                                }))
                            })
                        }
                        Column::App => {
                            html!("locale-cell-header", {
                                .prop("label", Column::App.to_string())
                                .prop("adminOnly", true)
                            })
                        }
                        Column::Element => {
                            html!("locale-cell-header", {
                                .prop("label", Column::Element.to_string())
                                .prop("adminOnly", true)
                            })
                        }
                        Column::Mock => {
                            html!("locale-cell-header", {
                                .prop("label", Column::Mock.to_string())
                                .prop("adminOnly", true)
                            })
                        }
                        Column::Actions => {
                            html!("locale-cell-header", {
                            })
                        },
                        Column::Bundle => {
                            html!("locale-cell-header", {
                                .prop("label", Column::Bundle.to_string())
                            })
                        },
                    }
                }))
            )
        })
    }
}
