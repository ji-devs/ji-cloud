use super::super::state::{SortKind, SortOrder, State};
use crate::locale::actions::AsStringExt;
use crate::locale::components::entry_row::EntryRow;
use crate::locale::components::select_columns;
use crate::locale::components::table_headers::TableHeaderDom;
use dominator::{clone, html, with_node, Dom};
use futures_signals::map_ref;
use futures_signals::signal::SignalExt;
use futures_signals::signal_vec::SignalVecExt;
use shared::domain::locale::ItemKind;
use std::rc::Rc;
use utils::events;
use utils::unwrap::UnwrapJiExt;
use uuid::Uuid;
use web_sys::HtmlSelectElement;

const STR_ADD_ENTRY: &str = "Add a text";

pub struct LocaleOuterDom {}

impl LocaleOuterDom {
    pub fn get_item_kind_name_by_id(map: &Vec<ItemKind>, id: Option<Uuid>) -> String {
        match id {
            Some(id) => map
                .iter()
                .find(|item_kind| item_kind.id == id)
                .unwrap_ji()
                .name
                .clone(),
            None => String::new(),
        }
    }

    pub fn render(state: Rc<State>) -> Dom {
        html!("empty-fragment", {
            .child(
                html!("locale-page", {
                    .property_signal("sortOrder", state.sort.signal_ref(|sort| sort.order.to_string()))
                    .property_signal("saving", state.saving_loader.is_loading())
                    .property_signal("columnsAmount", state.visible_columns.signal_vec_cloned().map(|_| 1).sum())
                    .children(&mut [
                        html!("select" => HtmlSelectElement, {
                            .property("slot", "bundles")
                            .attribute("multiple", "")
                            .with_node!(elem => {
                                .event(clone!(state, elem => move |_:events::Change| {
                                    state.loader.load(clone!(state, elem => async move {
                                        let options = elem.options();
                                        state.selected_bundles_change(&options).await;
                                    }))
                                }))
                            })
                            .children(
                                state.bundles.lock_ref().iter().map(|(e, selected)| {
                                    html!("option", {
                                        .property("text", e.name.to_string())
                                        .property("value", e.id.to_string())
                                        .property("selected", *selected)
                                    })
                                })
                            )
                        }),
                        html!("button-rect", {
                            .property("color", "blue")
                            .property("slot", "add-entry")
                            .text(STR_ADD_ENTRY)
                            // .child(html!("img", {
                            //     .attribute("src", "assets/add-icon.png")
                            // }))
                            .event(clone!(state => move |_event: events::Click| {
                                state.loader.load(clone!(state => async move {
                                    State::add_entry(state.clone()).await;
                                }))
                            }))
                        }),
                        select_columns::render(state.clone()),
                        TableHeaderDom::render(state.clone()),
                    ])
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
                                            SortKind::ItemKind => {
                                                let a = Self::get_item_kind_name_by_id(&state.item_kind_options, a.item_kind_id);
                                                let b = Self::get_item_kind_name_by_id(&state.item_kind_options, b.item_kind_id);
                                                a.cmp(&b)
                                            }
                                            SortKind::English => a.english.cmp(&b.english),
                                            SortKind::Hebrew => a.hebrew.cmp(&b.hebrew),
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
                                    let section = section.unwrap_or_default();
                                    *section_options.get(&section).unwrap_ji()
                                })),
                                let in_item_kind = state.item_kind_filter.signal_cloned().map(clone!(entry => move |item_kind_filter| {
                                    let item_kind_id = entry.lock_ref().item_kind_id;
                                    *item_kind_filter.get(&item_kind_id).unwrap_ji()
                                })),
                                let in_status = state.status_options.signal_cloned().map(clone!(entry => move |status_options| {
                                    let status = entry.lock_ref().status;
                                    *status_options.get(&status).unwrap_ji()
                                })) =>
                                *in_section && *in_item_kind && *in_status
                            }

                        }))
                        .map(clone!(state => move |entry| {
                            EntryRow::render(entry, state.clone())
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
