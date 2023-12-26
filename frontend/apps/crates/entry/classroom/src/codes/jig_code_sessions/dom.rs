use std::rc::Rc;

use dominator::{clone, html, Dom, DomBuilder};
use futures_signals::{signal::SignalExt, signal_vec::SignalVecExt};
use shared::domain::jig::codes::{JigCodeSessionResponse, JigPlaySessionModule};
use utils::component::Component;
use web_sys::ShadowRoot;

use super::CodeSessions;

impl Component<CodeSessions> for Rc<CodeSessions> {
    fn styles() -> &'static str {
        include_str!("./styles.css")
    }

    fn dom(&self, dom: DomBuilder<ShadowRoot>) -> DomBuilder<ShadowRoot> {
        let state = self;

        state.load_data();

        dom.child(html!("div", {
            .class("wrapper")
            .child(html!("h4", {
                .text("First round")
            }))
            .child(html!("div", {
                .class("round-items")
                .child(html!("div", {
                    .class("round-item")
                    .child(html!("p", {
                        .text("Shemesh/Sun")
                    }))
                    .child(html!("p", {
                        .text("tries")
                    }))
                }))
            }))
        }))
        .child_signal(state.jig.signal_cloned().map(clone!(state => move |jig| {
            jig.map(|jig| {
                html!("div", {
                    .text(&jig.id.to_string())
                    .text(&jig.jig_data.display_name.to_string())
                    .children_signal_vec(state.infos.signal_vec_cloned().map(clone!(state => move |info| {
                        state.render_session(info)
                    })))
                })
            })
        })))
    }
}

impl CodeSessions {
    fn render_session(self: &Rc<Self>, info: JigCodeSessionResponse) -> Dom {
        let state = self;

        html!("div", {
            .apply(|dom| {
                match info.info {
                    Some(info) => {
                        dom.children(info.modules.into_iter().map(|m| match m {
                            JigPlaySessionModule::Matching(session) => {
                                let module = state.modules.borrow().get(&session.module_id).cloned();
                                if module.is_some() {
                                    let module = module.unwrap().module.body.try_as_matching().unwrap().content.unwrap();
                                    super::modules::matching::Matching::new(module, session).render()
                                } else {
                                    html!("progress")
                                }
                            }
                        }))
                    },
                    None => dom,
                }
            })
        })
    }
}
