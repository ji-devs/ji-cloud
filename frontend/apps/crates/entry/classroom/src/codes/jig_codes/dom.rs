use components::asset_card::render_asset_card;
use dominator::{clone, html, DomBuilder, EventOptions};
use futures_signals::{
    signal::{not, Mutable, SignalExt},
    signal_vec::SignalVecExt,
};
use shared::domain::jig::TextDirection;
use std::rc::Rc;
use utils::{
    component::Component,
    date_formatters, events,
    routes::{ClassroomCodesRoute, ClassroomRoute, Route},
};
use web_sys::ShadowRoot;

use super::JigCodes;

impl Component<JigCodes> for Rc<JigCodes> {
    fn styles() -> &'static str {
        include_str!("./styles.css")
    }

    fn dom(&self, dom: DomBuilder<ShadowRoot>) -> DomBuilder<ShadowRoot> {
        let state = self;

        state.load_data();

        dom
            .child_signal(state.jig.signal_cloned().map(move |jig| {
                jig.map(|jig| {
                    render_asset_card(&jig.into(), Default::default())
                })
            }))
            .child(html!("div", {
                .class("codes")
                .child(html!("div", {
                    .class("header")
                    .child(html!("span", {
                        .class("cell")
                        .text("Name")
                    }))
                    .child(html!("span", {
                        .class("cell")
                        .text("Code")
                    }))
                    .child(html!("span", {
                        .class("cell")
                        .text("Created")
                    }))
                    .child(html!("span", {
                        .class("cell")
                        .text("Direction")
                    }))
                    .child(html!("span", {
                        .class("cell")
                        .text("Scoring & Assessment")
                    }))
                    // .child(html!("span", {
                    //     .class("cell")
                    //     .text("Drag assist")
                    // }))
                }))
                .children_signal_vec(state.codes.signal_vec_cloned().map(clone!(state => move |code| {
                    let editing = Mutable::new(false);
                    let original_name = Mutable::new(code.name.clone().unwrap_or_default());
                    let name = Mutable::new(original_name.get_cloned());
                    let route = Route::Classroom(ClassroomRoute::Codes(ClassroomCodesRoute::JigCodeSession(state.jig_id, code.index)));
                    html!("a", {
                        .class("code")
                        .prop("href", route.to_string())
                        .event_with_options(&dominator::EventOptions { preventable: true, bubbles: true }, clone!(editing => move |e:events::Click| {
                            e.prevent_default();
                            if !editing.get() {
                                route.go_to();
                            }
                        }))
                        .child(html!("div", {
                            .class("cell")
                            .class("name")
                            .child(html!("input", {
                                .prop_signal("readOnly", not(editing.signal()))
                                .prop_signal("value", name.signal_cloned())
                                .focused_signal(editing.signal())
                            }))
                            .child(html!("div", {
                                .class("actions")
                                .event_with_options(&EventOptions { preventable: true, bubbles: true }, move |e: events::Click| {
                                    e.stop_propagation();
                                    e.prevent_default();
                                })
                                .children_signal_vec(editing.signal().map(clone!(state => move |e| match e {
                                    false => vec![
                                        html!("fa-button", {
                                            .prop("icon", "fa-regular fa-pen-to-square")
                                            .prop("title", "Edit")
                                            .event(clone!(editing => move |_: events::Click| {
                                                editing.set(true);
                                            }))
                                        })
                                    ],
                                    true => vec![
                                        html!("fa-button", {
                                            .prop("icon", "fa-regular fa-floppy-disk")
                                            .prop("title", "Save")
                                            .event(clone!(state, editing, name, original_name => move |_: events::Click| {
                                                editing.set(false);
                                                original_name.set(name.get_cloned());
                                                state.save_name(code.index, name.get_cloned());
                                            }))
                                        }),
                                        html!("fa-button", {
                                            .prop("icon", "fa-regular fa-xmark")
                                            .prop("title", "Cancel")
                                            .event(clone!(editing, name, original_name => move |_: events::Click| {
                                                editing.set(false);
                                                name.set(original_name.get_cloned());
                                            }))
                                        }),
                                    ],
                                })).to_signal_vec())
                            }))
                        }))
                        .child(html!("span", {
                            .class("cell")
                            .text(&code.index.to_string())
                        }))
                        .child(html!("span", {
                            .class("cell")
                            .class("created-at")
                            .text(&date_formatters::year_month_day(&code.created_at))
                        }))
                        .child(html!("span", {
                            .class("cell")
                            .apply(|dom| {
                                match code.settings.direction {
                                    TextDirection::LeftToRight => {
                                        dom.prop("title", "Left to right")
                                    },
                                    TextDirection::RightToLeft => {
                                        dom.prop("title", "Right to left")
                                    },
                                }
                            })
                            .apply(|dom| {
                                match code.settings.direction {
                                    TextDirection::LeftToRight => {
                                        dom.child(html!("fa-icon", {
                                            .prop("icon", "fa-light fa-right")
                                        }))
                                    },
                                    TextDirection::RightToLeft => {
                                        dom.child(html!("fa-icon", {
                                            .prop("icon", "fa-light fa-left")
                                        }))
                                    },
                                }
                            })
                        }))
                        .child(html!("span", {
                            .class("cell")
                            .apply(|dom| {
                                match code.settings.scoring {
                                    true => dom.child(html!("fa-icon", {
                                        .prop("icon", "fa-solid fa-check")
                                    })),
                                    false => dom,
                                }
                            })
                        }))
                        // .child(html!("span", {
                        //     .class("cell")
                        //     .apply(|dom| {
                        //         match code.settings.drag_assist {
                        //             true => dom.child(html!("fa-icon", {
                        //                 .prop("icon", "fa-solid fa-check")
                        //             })),
                        //             false => dom,
                        //         }
                        //     })
                        // }))
                    })
                })))
            }))
    }
}
