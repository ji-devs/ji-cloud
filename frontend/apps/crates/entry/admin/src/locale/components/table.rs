use std::collections::BTreeMap;
use crate::locale::state::{ItemKind, Section, SortOrder};
use web_sys::HtmlSelectElement;
use crate::locale::state::{State, SortKind};
use std::rc::Rc;
use futures_signals::signal::SignalExt;
use futures_signals::signal_vec::SignalVecExt;
use futures_signals::map_ref;
use super::translation::TranslationRow;
use dominator::{Dom, html, clone, events, with_node};
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
                                            .class_signal("sorted", state.sort.signal_ref(|sort| sort.column == SortKind::Section))
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
                                            .class_signal("sorted", state.sort.signal_ref(|sort| sort.column == SortKind::ItemKind))
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
                                                    .children_signal_vec(state.item_kind_options.signal_cloned()
                                                        .map(|item_kind_options: BTreeMap<ItemKind, bool>| {
                                                            item_kind_options.iter().map(|(section_option, visible): (&ItemKind, &bool)| {
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
                                                        let mut item_kind_options = state.item_kind_options.lock_mut();

                                                        State::filter_change(&options, &mut item_kind_options);
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
                                            .class_signal("sorted", state.sort.signal_ref(|sort| sort.column == SortKind::English))
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
                                            .class_signal("sorted", state.sort.signal_ref(|sort| sort.column == SortKind::Status))
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
                                            .class_signal("sorted", state.sort.signal_ref(|sort| sort.column == SortKind::Comments))
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
                            map_ref! {
                                let in_section = state.section_options.signal_cloned().map(clone!(translation => move |section_options| {
                                    let section = translation.lock_ref().section.clone();
                                    let section = section.unwrap_or(String::new());
                                    *section_options.get(&section).unwrap()
                                })),
                                let in_item_kind = state.item_kind_options.signal_cloned().map(clone!(translation => move |item_kind_options| {
                                    let item_kind = translation.lock_ref().item_kind.clone();
                                    let item_kind = item_kind.unwrap_or(String::new());
                                    *item_kind_options.get(&item_kind).unwrap()
                                })),
                                let in_status = state.status_options.signal_cloned().map(clone!(translation => move |status_options| {
                                    let status = translation.lock_ref().status.clone();
                                    *status_options.get(&status).unwrap()
                                })) =>
                                *in_section && *in_item_kind && *in_status
                            }
                            
                        }))
                        .map(clone!(state => move |translation| {
                            TranslationRow::render(translation.clone(), state.clone())
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
