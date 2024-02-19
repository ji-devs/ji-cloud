use components::{
    module::_common::thumbnail::{ModuleThumbnail, ThumbnailFallback},
    player_popup::{PlayerPopup, PreviewPopupCallbacks},
};
use dominator::{clone, html, Dom, DomBuilder};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt},
};
use shared::domain::{
    asset::DraftOrLive,
    jig::codes::{JigCodeSessionResponse, JigPlaySessionModule},
    module::{ModuleBody, ModuleResponse, StableModuleId},
};
use std::{collections::HashMap, rc::Rc};
use utils::{asset::AssetPlayerOptions, component::Component, events};
use web_sys::ShadowRoot;

use super::{CodeSessions, JigWithModules};

impl Component<CodeSessions> for Rc<CodeSessions> {
    fn styles() -> &'static str {
        include_str!("./styles.css")
    }

    fn dom(&self, dom: DomBuilder<ShadowRoot>) -> DomBuilder<ShadowRoot> {
        let state = self;

        state.load_data();

        dom.child(html!("button-rect", {
            .class("preview-button")
            .prop("color", "blue")
            .prop("kind", "outline")
            .text("Preview JIG")
            .event(clone!(state => move |_: events::Click| {
                state.preview_open.set(true);
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
        html!("div", {
            .class("table")
            .style("--module-count", jig.jig.jig_data.modules.len().to_string())
            .child(html!("div", {
                .class("header")
                .child(html!("div", {
                    .class("cell")
                }))
                .child(html!("div", {
                    .class("cell")
                    .text("Student's Name")
                }))
                .children(jig.jig.jig_data.modules.iter().map(|module| {
                    html!("div", {
                        .class("cell")
                        .text(module.kind.as_str())
                    })
                }).collect::<Vec<_>>())
            }))
            .child(html!("div", {
                .class("thumbnails")
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
            }))
            .children(sessions.into_iter().map(clone!(state => move |session| {
                let open = Mutable::new(false);
                let sessions = session.info.unwrap().modules.into_iter().map(|module| {
                    let stable_module_id = match &module {
                        JigPlaySessionModule::Matching(module) => module.stable_module_id,
                    };
                    (stable_module_id, module)
                }).collect::<HashMap<StableModuleId, JigPlaySessionModule>>();
                html!("div", {
                    .class("session")
                    .class_signal("open", open.signal())
                    .child(html!("div", {
                        .class("cell")
                        .child(html!("fa-icon", {
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
                    .children(jig.jig.jig_data.modules.iter().map(|module| {
                        html!("div", {
                            .class("cell")
                            .apply(|dom| {
                                let stable_module_id = module.stable_id;
                                let module = jig.modules.get(&stable_module_id).unwrap().clone();
                                if let Some(session) = sessions.get(&stable_module_id) {
                                    dom
                                        .text(&state.get_count(&session))
                                        .child_signal(open.signal().map(clone!(state, session => move |open| {
                                            open.then(|| {
                                                state.render_session(&module, &session.clone())
                                            })
                                        })))
                                } else {
                                    dom
                                }
                            })
                        })
                    }).collect::<Vec<_>>())
                })
            })))
        })
    }

    fn render_session(
        self: &Rc<Self>,
        module: &ModuleResponse,
        session: &JigPlaySessionModule,
    ) -> Dom {
        html!("div", {
            .apply(|dom| {
                match (&module.module.body, &session) {
                    (ModuleBody::Matching(module), JigPlaySessionModule::Matching(session)) => {
                        dom
                        .child(super::modules::matching::render_matching(&module.content.clone().unwrap(), &session))
                    },
                    _ => dom
                }
            })
        })
    }

    fn get_count(self: &Rc<Self>, session: &JigPlaySessionModule) -> String {
        match &session {
            JigPlaySessionModule::Matching(session) => {
                super::modules::matching::get_matching_count(&session)
            }
        }
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
