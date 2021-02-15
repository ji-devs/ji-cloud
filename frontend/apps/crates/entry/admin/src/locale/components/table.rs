use crate::locale::state::{TranslationStatus, SortOrder};
use web_sys::HtmlSelectElement;
use crate::locale::state::{State, SortKind};
use std::rc::Rc;
use futures_signals::signal::SignalExt;
use futures_signals::signal_vec::SignalVecExt;
use super::translation::TranslationRow;
use dominator::{Dom, html, clone, events, with_node};
use super::select_columns::SelectColumns;
use strum::IntoEnumIterator;


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
                    .child(
                        html!("div", {
                            .class("ftl-row")
                            .children(&mut [
                                html!("div", {
                                    .class("ftl-sup-header-cell")
                                }),
                                html!("div", {
                                    .class("ftl-sup-header-cell")
                                    .children(&mut [
                                        html!("button", {
                                            .class("link-button")
                                            .text("Sort")
                                            .event(clone!(state => move |_event: events::Click| {
                                                state.sort_clicked(SortKind::Section);
                                            }))
                                        }),
                                        // html!("span", {
                                        //     .text("|")
                                        // }),
                                        html!("div", {
                                            .class("filter")
                                            // .text("Filter")
                                            .child(html!("select" => HtmlSelectElement, {
                                                .with_node!(elem => {
                                                    .attribute("multiple", "")
                                                    .style("width", "100px")
                                                    .children_signal_vec(state.sections.signal_vec_cloned()
                                                        .map(clone!(state => move |section| {
                                                            html!("option", {
                                                                .property("value", &section)
                                                                .property("selected", state.filters.lock_ref().section.contains(&section))
                                                                .text(&section)
                                                            })
                                                        })))
                                                    .event(clone!(state => move |_event: events::Change| {
                                                        let options = elem.options();
                                                        let mut filters = state.filters.lock_mut();

                                                        State::filter_change(&options, &mut filters.section);
                                                    }))
                                                })
                                            }))
                                        }),
                                    ])
                                }),
                                html!("div", {
                                    .class("ftl-sup-header-cell")
                                    .children(&mut [
                                        html!("button", {
                                            .class("link-button")
                                            .text("Sort")
                                            .event(clone!(state => move |_event: events::Click| {
                                                state.sort_clicked(SortKind::ItemKind);
                                            }))
                                        }),
                                        // html!("span", {
                                        //     .text("|")
                                        // }),
                                        html!("div", {
                                            .class("filter")
                                            // .text("Filter")
                                            .child(html!("select" => HtmlSelectElement, {
                                                .with_node!(elem => {
                                                    .attribute("multiple", "")
                                                    .style("width", "100px")
                                                    .children_signal_vec(state.item_kinds.signal_vec_cloned()
                                                        .map(clone!(state => move |item_kind| {
                                                            html!("option", {
                                                                .property("value", &item_kind)
                                                                .property("selected", state.filters.lock_ref().item_kind.contains(&item_kind))
                                                                .text(&item_kind)
                                                            })
                                                        })))
                                                    .event(clone!(state => move |_event: events::Change| {
                                                        let options = elem.options();
                                                        let mut filters = state.filters.lock_mut();

                                                        State::filter_change(&options, &mut filters.item_kind);
                                                    }))
                                                })
                                            }))
                                        }),
                                    ])
                                }),
                                html!("div", {
                                    .class("ftl-sup-header-cell")
                                    .children(&mut [
                                        html!("button", {
                                            .class("link-button")
                                            .text("Sort")
                                            .event(clone!(state => move |_event: events::Click| {
                                                state.sort_clicked(SortKind::English);
                                            }))
                                        }),
                                    ])
                                }),
                                html!("div", {
                                    .class("ftl-sup-header-cell")
                                    .children(&mut [
                                        html!("button", {
                                            .class("link-button")
                                            .text("Sort")
                                            .event(clone!(state => move |_event: events::Click| {
                                                state.sort_clicked(SortKind::Status);
                                            }))
                                        }),
                                        // html!("span", {
                                        //     .text("|")
                                        // }),
                                        html!("div", {
                                            .class("filter")
                                            // .text("Filter")
                                            .child(html!("select" => HtmlSelectElement, {
                                                .with_node!(elem => {
                                                    .attribute("multiple", "")
                                                    .style("width", "100px")
                                                    .children(
                                                        TranslationStatus::iter().map(|o| {
                                                            html!("option", {
                                                                .property("text", o.to_string())
                                                                .property("value", o.to_string())
                                                                .property("selected", state.filters.lock_ref().status.contains(&o))
                                                            })
                                                        })
                                                    )
                                                    .event(clone!(state => move |_event: events::Change| {
                                                        let options = elem.options();
                                                        let mut filters = state.filters.lock_mut();

                                                        State::filter_change(&options, &mut filters.status);
                                                    }))
                                                })
                                            }))
                                        }),
                                    ])
                                }),
                                html!("div", {
                                    .class("ftl-sup-header-cell")
                                }),
                                html!("div", {
                                    .class("ftl-sup-header-cell")
                                    .children(&mut [
                                        html!("button", {
                                            .class("link-button")
                                            .text("Sort")
                                            .event(clone!(state => move |_event: events::Click| {
                                                state.sort_clicked(SortKind::Comments);
                                            }))
                                        }),
                                    ])
                                }),
                                html!("div", {
                                    .class("ftl-sup-header-cell")
                                }),
                                html!("div", {
                                    .class("ftl-sup-header-cell")
                                }),
                                html!("div", {
                                    .class("ftl-sup-header-cell")
                                }),
                                html!("div", {
                                    .class("ftl-sup-header-cell")
                                }),
                            ])
                        })
                    )
                    .child(
                        html!("div", {
                            .class("ftl-row")
                            .children(&mut [
                                html!("div", {
                                    .class("ftl-header-cell")
                                    .text("ID")
                                }),
                                html!("div", {
                                    .class("ftl-header-cell")
                                    .text("Section")
                                }),
                                html!("div", {
                                    .class("ftl-header-cell")
                                    .text("Translation Kind")
                                }),
                                html!("div", {
                                    .class("ftl-header-cell")
                                    .text("English")
                                }),
                                html!("div", {
                                    .class("ftl-header-cell")
                                    .text("Status")
                                }),
                                html!("div", {
                                    .class("ftl-header-cell")
                                    .text("Zeplin reference")
                                }),
                                html!("div", {
                                    .class("ftl-header-cell")
                                    .text("Comments")
                                }),
                                html!("div", {
                                    .class("ftl-header-cell")
                                    .class("office-use-only")
                                    .text("App")
                                }),
                                html!("div", {
                                    .class("ftl-header-cell")
                                    .class("office-use-only")
                                    .text("Element")
                                }),
                                html!("div", {
                                    .class("ftl-header-cell")
                                    .class("office-use-only")
                                    .text("Mock")
                                }),
                                html!("div", {
                                    .class("ftl-header-cell")
                                }),
                            ])
                        })
                    )
                    .children_signal_vec(state.sort
                        .signal_cloned()
                        .switch(clone!(state => move |sort| {
                            state.translations
                                .signal_vec_cloned()
                                .to_signal_map(clone!(state => move |translations| {
                                    let mut translations = translations.to_vec();

                                    translations.sort_by(|a, b| {
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

                                    translations
                                }))
                        }))
                        .to_signal_vec()
                        .filter_signal_cloned(clone!(state => move |translation| {
                            state.filters.signal_cloned().map(clone!(translation => move |filters| {
                                let section = translation.lock_ref().section.clone();
                                let section = section.unwrap_or(String::new());
                                if !filters.section.contains(&section) {
                                    return false
                                };

                                let item_kind = translation.lock_ref().item_kind.clone();
                                let item_kind = item_kind.unwrap_or(String::new());
                                if !filters.item_kind.contains(&item_kind) {
                                    return false
                                };

                                let status = translation.lock_ref().status.clone();
                                if !filters.status.contains(&status) {
                                    return false
                                }
                                true
                            }))
                        }))
                        .map(clone!(state => move |translation| {
                            TranslationRow::render(translation.clone(), state.clone())
                        })))
                }),

                html!("datalist", {
                    .property("id", "sections")
                    .children_signal_vec(state.sections.signal_vec_cloned()
                        .map(|section| {
                            html!("option", {
                                .property("value", section)
                            })
                        }))
                }),

                html!("datalist", {
                    .property("id", "translation-kinds")
                    .children_signal_vec(state.item_kinds.signal_vec_cloned()
                        .map(|item_kind| {
                            html!("option", {
                                .property("value", item_kind)
                            })
                        }))
                }),

                SelectColumns::render(state.clone()),
            ])
        })
    }
}
