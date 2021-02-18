use std::collections::BTreeMap;
use std::rc::Rc;
use web_sys::HtmlSelectElement;
use futures_signals::signal::SignalExt;
use dominator::{Dom, html, with_node, clone};
use utils::events;
use crate::locale::state::{State, SortKind, Section, ItemKind};

pub struct TableHeaderDom {

}

impl TableHeaderDom {
    pub fn render(state: Rc<State>) -> Dom {
        // just a placeholder because I don't know how to return 2 children
        html!("div", {
            .style("display", "contents")
            .children(&mut [
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
                }),
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
                            .text("Item Kind")
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
            ])
        })
    }
}
