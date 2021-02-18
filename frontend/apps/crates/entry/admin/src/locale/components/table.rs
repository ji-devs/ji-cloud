use crate::locale::components::table_headers::TableHeaderDom;
use std::collections::BTreeMap;
use crate::locale::state::{Section, SortOrder};
use crate::locale::state::{State, SortKind};
use std::rc::Rc;
use futures_signals::signal::SignalExt;
use futures_signals::signal_vec::SignalVecExt;
use futures_signals::map_ref;
use super::entry_row::EntryRow;
use dominator::{Dom, html, clone};
use super::select_columns::SelectColumns;


#[derive(Debug)]
pub struct TableComponent {

}

impl TableComponent {
    pub fn render(state: Rc<State>) -> Dom {
        // just a placeholder because I don't know how to return 2 children
        html!("div", {
            // inline style because I'd really like to remove this element altogether
            .style("display", "contents")
            .children(&mut [
                html!("div", {
                    .class("ftl-table")
                    .class_signal("asc", state.sort.signal_ref(|sort| sort.order == SortOrder::Asc))
                    .class_signal("desc", state.sort.signal_ref(|sort| sort.order == SortOrder::Desc))
                    .child(TableHeaderDom::render(state.clone()))
                    .children_signal_vec(state.sort
                        .signal_cloned()
                        .switch(clone!(state => move |sort| {
                            state.entries
                                .signal_vec_cloned()
                                .to_signal_map(clone!(state => move |entries| {
                                    let mut entries = entries.to_vec();

                                    entries.sort_by(|a, b| {
                                        let a = a.lock_ref();
                                        let b = b.lock_ref();
                                        let mut ord = match sort.column {
                                            SortKind::Section => a.section.cmp(&b.section),
                                            SortKind::ItemKind => a.item_kind.cmp(&b.item_kind),
                                            SortKind::English => a.english.cmp(&b.english),
                                            SortKind::Status => a.status.to_string().cmp(&b.status.to_string()),
                                            SortKind::Comments => a.comments.cmp(&b.comments),
                                        };

                                        if sort.order == SortOrder::Asc {
                                            ord = ord.reverse();
                                        }

                                        ord
                                    });

                                    entries
                                }))
                        }))
                        .to_signal_vec()
                        .filter_signal_cloned(clone!(state => move |entry| {
                            map_ref! {
                                let in_section = state.section_options.signal_cloned().map(clone!(entry => move |section_options| {
                                    let section = entry.lock_ref().section.clone();
                                    let section = section.unwrap_or(String::new());
                                    *section_options.get(&section).unwrap()
                                })),
                                let in_item_kind = state.item_kind_options.signal_cloned().map(clone!(entry => move |item_kind_options| {
                                    let item_kind = entry.lock_ref().item_kind.clone();
                                    let item_kind = item_kind.unwrap_or(String::new());
                                    *item_kind_options.get(&item_kind).unwrap()
                                })),
                                let in_status = state.status_options.signal_cloned().map(clone!(entry => move |status_options| {
                                    let status = entry.lock_ref().status.clone();
                                    *status_options.get(&status).unwrap()
                                })) =>
                                *in_section && *in_item_kind && *in_status
                            }
                            
                        }))
                        .map(clone!(state => move |entry| {
                            EntryRow::render(entry.clone(), state.clone())
                        })))
                }),

                html!("datalist", {
                    .property("id", "sections")
                    .children_signal_vec(state.section_options.signal_cloned()
                        .map(|section_options: BTreeMap<Section, bool>| {
                            section_options.iter().map(|(section_option, _)| {
                                html!("option", {
                                    .property("value", section_option)
                                    .text(&section_option)
                                })
                            }).collect()
                        })
                        .to_signal_vec())
                }),

                html!("datalist", {
                    .property("id", "item-kinds")
                    .children_signal_vec(state.item_kind_options.signal_cloned()
                        .map(|item_kind_options: BTreeMap<Section, bool>| {
                            item_kind_options.iter().map(|(item_kind_option, _)| {
                                html!("option", {
                                    .property("value", item_kind_option)
                                    .text(&item_kind_option)
                                })
                            }).collect()
                        })
                        .to_signal_vec())
                }),

                SelectColumns::render(state.clone()),
            ])
        })
    }
}
