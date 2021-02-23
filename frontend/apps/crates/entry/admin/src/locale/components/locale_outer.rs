use crate::locale::actions::{map_to_js, b_tree_to_js};
use crate::locale::components::table_headers::TableHeaderDom;
use crate::locale::components::entry_row::EntryRow;
use dominator::{Dom, html, clone};
use std::rc::Rc;
use super::super::state::*;
use super::super::events;
use futures_signals::signal::SignalExt;
use futures_signals::signal_vec::SignalVecExt;
use futures_signals::map_ref;


pub struct LocaleOuterDom {

}

impl LocaleOuterDom {
    pub fn render(state: Rc<State>) -> Dom {
        html!("empty-fragment", {
            .child(
                html!("locale-page", {
                    .property("bundles", map_to_js(&state.bundles))
                    .property("columns", b_tree_to_js(&state.columns))
                    .property_signal("sortOrder", state.sort.signal_ref(|sort| sort.order.to_string()))
                    .property_signal("saving", state.saving_loader.is_loading())
                    .event(clone!(state => move |_event: events::AddEntryEvent| {
                        state.loader.load(clone!(state => async move {
                            state.add_entry().await;
                        }))
                    }))
                    .event(clone!(state => move |event: events::SelectedBundleChangeEvent| {
                        state.loader.load(clone!(state => async move {
                            state.selected_bundles_change(event.bundles()).await;
                        }))
                    }))
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
                })
            )
            .child(
                html!("window-loader-block", {
                    .property_signal("visible", state.loader.is_loading())
                })
            )
        })
    }
}
