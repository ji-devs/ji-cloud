use crate::locale::actions::b_tree_to_js;
use std::rc::Rc;
use futures_signals::signal::SignalExt;
use dominator::{Dom, html, clone};
use super::super::events;
use crate::locale::state::{State, SortKind, EntryStatus};
use std::str::FromStr;

pub struct TableHeaderDom {

}

impl TableHeaderDom {
    pub fn render(state: Rc<State>) -> Dom {
        html!("locale-row", {
            .property("slot", "rows")
            .children(&mut [
                html!("locale-cell-header", {
                    .property("label", "ID")
                }),
                html!("locale-cell-header", {
                    .property("label", "Section")
                    .property("sortable", true)
                    .property_signal("sorted", state.sort.signal_ref(|sort| sort.column == SortKind::Section))
                    .event(clone!(state => move |_event: events::SortEvent| {
                        state.sort_clicked(SortKind::Section);
                    }))
                    .property_signal("filterOptions", state.section_options.signal_cloned().map(|o| b_tree_to_js(&o)))
                    .event(clone!(state => move |event: events::FilterEvent| {
                        state.section_options.set(event.options());
                    }))
                }),
                html!("locale-cell-header", {
                    .property("label", "Item Kind")
                    .property("sortable", true)
                    .property_signal("sorted", state.sort.signal_ref(|sort| sort.column == SortKind::ItemKind))
                    .event(clone!(state => move |_event: events::SortEvent| {
                        state.sort_clicked(SortKind::ItemKind);
                    }))
                    .property_signal("filterOptions", state.item_kind_options.signal_cloned().map(|o| b_tree_to_js(&o)))
                    .event(clone!(state => move |event: events::FilterEvent| {
                        state.item_kind_options.set(event.data());
                    }))
                }),
                html!("locale-cell-header", {
                    .property("label", "English")
                    .property("sortable", true)
                    .property_signal("sorted", state.sort.signal_ref(|sort| sort.column == SortKind::English))
                    .event(clone!(state => move |_event: events::SortEvent| {
                        state.sort_clicked(SortKind::English);
                    }))
                }),
                html!("locale-cell-header", {
                    .property("label", "Status")
                    .property("sortable", true)
                    .property_signal("sorted", state.sort.signal_ref(|sort| sort.column == SortKind::Status))
                    .event(clone!(state => move |_event: events::SortEvent| {
                        state.sort_clicked(SortKind::Status);
                    }))
                    .property_signal("filterOptions", state.status_options.signal_cloned().map(|o| b_tree_to_js(&o)))
                    .event(clone!(state => move |event: events::FilterEvent| {
                        state.status_options.set(event.options()
                            .iter()
                            .map(|(o, selected)| (EntryStatus::from_str(o).unwrap_or(EntryStatus::Approved), *selected))
                            .collect()
                        );
                    }))
                }),
                html!("locale-cell-header", {
                    .property("label", "Zeplin reference")
                }),
                html!("locale-cell-header", {
                    .property("label", "Comments")
                    .property("sortable", true)
                    .property_signal("sorted", state.sort.signal_ref(|sort| sort.column == SortKind::Comments))
                    .event(clone!(state => move |_event: events::SortEvent| {
                        state.sort_clicked(SortKind::Comments);
                    }))
                }),
                html!("locale-cell-header", {
                    .property("label", "App")
                    .property("adminonly", true)
                }),
                html!("locale-cell-header", {
                    .property("label", "Element")
                    .property("adminonly", true)
                }),
                html!("locale-cell-header", {
                    .property("label", "Mock")
                    .property("adminonly", true)
                }),
                html!("locale-cell-header", {
                }),
            ])
        })
    }
}
