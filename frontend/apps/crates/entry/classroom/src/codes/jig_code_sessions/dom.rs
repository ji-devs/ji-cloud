use components::{
    module::_common::thumbnail::{ModuleThumbnail, ThumbnailFallback},
    player_popup::{PlayerPopup, PreviewPopupCallbacks},
};
use dominator::{clone, html, with_node, Dom, DomBuilder};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt},
};
use shared::domain::{
    asset::DraftOrLive,
    jig::codes::{
        JigCodeSessionResponse, JigPlaySessionModule, JigPlaySessionModuleGetPointsEarned,
    },
    module::{ModuleBody, ModuleResponse, StableModuleId},
};
use std::{collections::HashMap, rc::Rc};
use utils::{
    asset::AssetPlayerOptions,
    clipboard,
    component::Component,
    date_formatters, events,
    init::settings::SETTINGS,
    on_click_go_to_url,
    routes::{ClassroomCodesRoute, ClassroomRoute, KidsRoute, Route},
    unwrap::UnwrapJiExt,
};
use web_sys::{window, HtmlElement, ShadowRoot};

use super::{CodeSessions, JigWithModules};

impl Component<CodeSessions> for Rc<CodeSessions> {
    fn styles() -> &'static str {
        include_str!("./styles.css")
    }

    fn dom(&self, dom: DomBuilder<ShadowRoot>) -> DomBuilder<ShadowRoot> {
        let state = self;

        state.load_data();

        dom.child(html!("header", {
            .child(html!("button-rect", {
                .class("back-button")
                .prop("kind", "text")
                .prop("color", "blue")
                .apply(move |dom| on_click_go_to_url!(dom, {
                    Route::Classroom(ClassroomRoute::Codes(ClassroomCodesRoute::JigCodes(state.jig_id)))
                }))
                .child(html!("fa-icon", {
                   .prop("icon", "fa-regular fa-chevron-left")
                }))
                .text("Back")
            }))
            .child(html!("h2", {
                .class("code")
                .text(&state.code.to_string())
                .child(html!("fa-button", {
                    .class("copy")
                    .prop("title", "Copy code")
                    .prop("icon", "fa-regular fa-copy")
                    .event(clone!(state => move |_: events::Click| {
                        clipboard::write_text(&state.code.to_string());
                    }))
                }))
            }))
            .child(html!("h3", {
                .class("jig-name")
                .text_signal(state.jig.signal_ref(|jig| {
                    jig.as_ref().map(|jig| jig.jig.jig_data.display_name.clone()).unwrap_or_default()
                }))
            }))
            .child(html!("button-rect", {
                .class("preview-button")
                .prop("color", "blue")
                .prop("kind", "outline")
                .text("Preview JIG")
                .event(clone!(state => move |_: events::Click| {
                    state.preview_open.set(true);
                }))
            }))
            .child(html!("p", {
                .class("code-link")
                .text("jigzi.org")
                .text(&Route::Kids(KidsRoute::StudentCode(Some(state.code.to_string()))).to_string())
                .child(html!("fa-button", {
                    .class("copy")
                    .prop("title", "Copy link")
                    .prop("icon", "fa-regular fa-copy")
                    .event(clone!(state => move |_: events::Click| {
                        let url = SETTINGS.get().unwrap_ji().remote_target.pages_url() + &Route::Kids(KidsRoute::StudentCode(Some(state.code.to_string()))).to_string();
                        clipboard::write_text(&url);
                    }))
                }))
            }))
            .child(html!("div", {
                .class("export-and-qr")
                .child(html!("button-rect", {
                    .class("qr-button")
                    .prop("color", "blue")
                    .prop("kind", "text")
                    .text("Show QR code")
                    .event(clone!(state => move |_: events::Click| {
                        state.show_qr_code();
                    }))
                }))
                .child(html!("button-rect", {
                    .class("export-button")
                    .prop("color", "blue")
                    .prop("kind", "text")
                    .text("Export CSV")
                    .event(clone!(state => move |_: events::Click| {
                        state.export_sessions();
                    }))
                }))
            }))
        }))
        .child_signal(state.module_and_session_signal().map(
            clone!(state => move |jig_and_session| {
                Some(match jig_and_session {
                    None => {
                        html!("progress")
                    },
                    Some((jig, sessions)) => {
                        state.render_loaded(jig, sessions)
                    },
                })
            }),
        ))
        .child_signal(
            state
                .preview_open
                .signal_cloned()
                .map(clone!(state => move|preview_open| {
                    preview_open.then(|| {
                        let close = clone!(state => move || {
                            state.preview_open.set(false);
                        });
                        PlayerPopup::new(
                            state.jig_id.into(),
                            None,
                            None,
                            AssetPlayerOptions::Jig(Default::default()),
                            PreviewPopupCallbacks::new(close)
                        ).render(None)
                    })
                })),
        )
        .child_signal(state.qr_dialog.signal_ref(move |qr_dialog| {
            qr_dialog.as_ref().map(move |qr_dialog| {
                qr_dialog.render()
            })
        }))
    }
}

impl CodeSessions {
    fn render_loaded(
        self: &Rc<Self>,
        jig: JigWithModules,
        sessions: Vec<JigCodeSessionResponse>,
    ) -> Dom {
        let state = self;
        let jig_id = jig.jig.id;
        html!("div" => HtmlElement, {
            .class("table")
            .with_node!(table_elem => {
                .apply(|dom| {
                    // this code enables mouse drag
                    let is_dragging = Mutable::new(false);
                    let doc_elem = window().unwrap_ji().document().unwrap_ji().document_element().unwrap_ji();
                    dom
                        .event(clone!(is_dragging => move |_: events::PointerDown| {
                            is_dragging.set(true);
                        }))
                        .global_event(clone!(is_dragging => move |_: events::PointerUp| {
                            is_dragging.set(false);
                        }))
                        .global_event(clone!(is_dragging => move |_: events::PointerCancel| {
                            is_dragging.set(false);
                        }))
                        .global_event(clone!(is_dragging => move |e: events::PointerMove| {
                            if is_dragging.get() {
                                table_elem.set_scroll_left(table_elem.scroll_left() - e.movement_x());
                                doc_elem.set_scroll_top(doc_elem.scroll_top() - e.movement_y());
                            }
                        }))
                })
            })
            .style("--module-count", jig.jig.jig_data.modules.len().to_string())
            .child(html!("div", {
                .class("header")
                .child(html!("div", {
                    .class("cell")
                }))
                .child(html!("div", {
                    .class("cell")
                    .text("Student Name")
                }))
                .child(html!("div", {
                    .class("cell")
                    .text("Started")
                }))
                .child(html!("div", {
                    .class("cell")
                    .text("Ended")
                }))
                .children(jig.jig.jig_data.modules.iter().map(|module| {
                    html!("div", {
                        .class("cell")
                        .text(module.kind.display_name())
                    })
                }).collect::<Vec<_>>())
                .child(html!("div", {
                    .class("cell")
                    .text("Total")
                }))
            }))
            .child(html!("div", {
                .class("thumbnails")
                .child(html!("div", {
                    .class("cell")
                }))
                .child(html!("div", {
                    .class("cell")
                }))
                .child(html!("div", {
                    .class("cell")
                }))
                .child(html!("div", {
                    .class("cell")
                }))
                .children(jig.jig.jig_data.modules.iter().map(|module| {
                    html!("div", {
                        .class("cell")
                        .child(html!("div", {
                            .class("thumbnail")
                            .child(ModuleThumbnail::new(
                                jig_id.into(),
                                Some(module.clone()),
                                ThumbnailFallback::Module,
                                DraftOrLive::Live
                            ).render(Some("image")))
                        }))
                    })
                }).collect::<Vec<_>>())
                .child(html!("div", {
                    .class("cell")
                }))
            }))
            .children(sessions.into_iter().map(clone!(state => move |session| {
                let open = Mutable::new(false);
                let total_points_earned = session.info.as_ref().map(|i| i.get_points_earned());
                let visited = session.info.as_ref().map(|i| i.visited.clone()).unwrap_or_default();
                let updated_since = match (&jig.jig.published_at, &session.finished_at) {
                    (Some(jig_published_at), Some(session_finished_at)) if jig_published_at > session_finished_at => {
                        true
                    },
                    _ => false,
                };
                let sessions = session.info.unwrap().modules.into_iter().map(|module| {
                    let stable_module_id = module.stable_module_id();
                    (stable_module_id, module)
                }).collect::<HashMap<StableModuleId, JigPlaySessionModule>>();
                html!("div", {
                    .class("session")
                    .class_signal("open", open.signal())
                    .child(html!("div", {
                        .class("cell")
                        .child(html!("fa-icon", {
                            .class("open-icon")
                            .prop("icon", "fa-regular fa-angle-right")
                        }))
                        .event(clone!(open => move |_: events::Click| {
                            open.replace_with(|open| {
                                !*open
                            });
                        }))
                    }))
                    .child(html!("div", {
                        .class("cell")
                        .text(&session.players_name.unwrap_or_default())
                    }))
                    .child(html!("div", {
                        .class("cell")
                        .class("time")
                        .text(&date_formatters::year_month_day(&session.started_at))
                        .child(html!("br"))
                        .text(&date_formatters::hour_minute(&session.started_at))
                    }))
                    .child(html!("div", {
                        .class("cell")
                        .class("time")
                        .text(&session.finished_at.map(|t| date_formatters::year_month_day(&t)).unwrap_or_default())
                        .child(html!("br"))
                        .text(&session.finished_at.map(|t| date_formatters::hour_minute(&t)).unwrap_or_default())
                    }))
                    .children(jig.jig.jig_data.modules.iter().map(|module| {
                        html!("div", {
                            .class("cell")
                            .apply(|dom| {
                                let stable_module_id = module.stable_id;
                                let module = jig.modules.get(&stable_module_id).unwrap().clone();
                                if let Some(session) = sessions.get(&stable_module_id) {
                                    dom
                                        .text(&session.get_points_earned().to_string())
                                        .child_signal(open.signal().map(clone!(state, session => move |open| {
                                            open.then(|| {
                                                state.render_session(&module, &session.clone(), updated_since)
                                            })
                                        })))
                                } else if visited.contains(&stable_module_id) {
                                    dom
                                        .prop("title", "Visited")
                                        .child(html!("fa-icon", {
                                            .class("visited-icon")
                                            .prop("icon", "fa-solid fa-check")
                                        }))
                                } else {
                                    dom
                                }
                            })
                        })
                    }).collect::<Vec<_>>())
                    .child(html!("div", {
                        .class("cell")
                        .class("total")
                        .child(html!("span", {
                            .text(&total_points_earned.as_ref().map(|p| format!("{}%", p.percent())).unwrap_or_default())
                        }))
                        .child(html!("span", {
                            .text(&total_points_earned.map(|p| p.to_string()).unwrap_or_default())
                        }))
                    }))
                })
            })))
        })
    }

    fn render_session(
        self: &Rc<Self>,
        module: &ModuleResponse,
        session: &JigPlaySessionModule,
        updated_since: bool,
    ) -> Dom {
        html!("div", {
            .apply_if(updated_since, |dom| {
                dom.child(html!("span", {
                    .class("updated-since")
                    .text("Updated since")
                    .child(html!("fa-icon", {
                        .prop("icon", "fa-regular fa-circle-info")
                    }))
                    .prop("title", "This JIG was updated since this student played it. Some data might have changed since.")
                }))
            })
            .apply(|dom| {
                match (&module.module.body, &session) {
                    (ModuleBody::Matching(module), JigPlaySessionModule::Matching(session)) => {
                        dom
                        .child(super::modules::matching::render_matching(&module.content.clone().unwrap(), &session))
                    },
                    (ModuleBody::CardQuiz(module), JigPlaySessionModule::CardQuiz(session)) => {
                        dom
                        .child(super::modules::card_quiz::render_card_quiz(&module.content.clone().unwrap(), &session))
                    },
                    (ModuleBody::DragDrop(module), JigPlaySessionModule::DragDrop(session)) => {
                        dom
                        .child(super::modules::drag_drop::render_drag_drop(&module.content.clone().unwrap(), &session))
                    },
                    (ModuleBody::FindAnswer(module), JigPlaySessionModule::FindAnswer(session)) => {
                        dom
                        .child(super::modules::find_answer::render_find_answer(&module.content.clone().unwrap(), &session))
                    },
                    _ => dom
                }
            })
        })
    }

    fn module_and_session_signal(
        &self,
    ) -> impl Signal<Item = Option<(JigWithModules, Vec<JigCodeSessionResponse>)>> {
        map_ref! {
            let jig = self.jig.signal_cloned(),
            let infos = self.infos.signal_cloned() => move {
                jig.clone().map(move |jig| {
                    // TODO: get rid of .to_vec()
                    (jig, infos.to_vec())
                })
            }
        }
    }
}
