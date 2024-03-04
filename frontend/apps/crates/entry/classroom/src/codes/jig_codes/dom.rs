use components::asset_card::render_asset_card;
use dominator::{clone, html, DomBuilder};
use futures_signals::{signal::SignalExt, signal_vec::SignalVecExt};
use shared::domain::jig::TextDirection;
use std::rc::Rc;
use utils::{
    component::Component,
    date_formatters, link,
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
                    // .child(html!("span", {
                    //     .class("cell")
                    //     .text("Name")
                    // }))
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
                        .text("Settings")
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
                    link!(Route::Classroom(ClassroomRoute::Codes(ClassroomCodesRoute::JigCodeSession(state.jig_id, code.index))), {
                        .class("code")
                        // .child(html!("span", {
                        //     .class("cell")
                        //     .text(&code.name.unwrap_or_default())
                        // }))
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
