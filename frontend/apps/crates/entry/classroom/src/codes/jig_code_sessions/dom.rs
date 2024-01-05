use dominator::{clone, events, html, Dom, DomBuilder};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt},
};
use shared::domain::{
    jig::codes::{JigCodeSessionResponse, JigPlaySessionModule},
    module::{ModuleBody, ModuleId, ModuleResponse},
};
use std::{collections::HashMap, rc::Rc};
use utils::component::Component;
use web_sys::ShadowRoot;

use super::{CodeSessions, JigWithModules};

impl Component<CodeSessions> for Rc<CodeSessions> {
    fn styles() -> &'static str {
        include_str!("./styles.css")
    }

    fn dom(&self, dom: DomBuilder<ShadowRoot>) -> DomBuilder<ShadowRoot> {
        let state = self;

        state.load_data();

        dom.child_signal(state.module_and_session_signal().map(
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
    }
}

impl CodeSessions {
    fn render_loaded(
        self: &Rc<Self>,
        jig: JigWithModules,
        sessions: Vec<JigCodeSessionResponse>,
    ) -> Dom {
        let state = self;
        html!("div", {
            .class("table")
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
            .children(sessions.into_iter().map(clone!(state => move |session| {
                let open = Mutable::new(false);
                let sessions = session.info.unwrap().modules.into_iter().map(|module| {
                    let module_id = match &module {
                        JigPlaySessionModule::Matching(module) => module.module_id,
                    };
                    (module_id, module)
                }).collect::<HashMap<ModuleId, JigPlaySessionModule>>();
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
                                let module_id = module.id;
                                let module = jig.modules.get(&module_id).unwrap().clone();
                                if let Some(session) = sessions.get(&module_id) {
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
