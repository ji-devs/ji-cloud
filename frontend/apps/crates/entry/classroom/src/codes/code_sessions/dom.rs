use std::rc::Rc;

use dominator::{clone, html, Dom, DomBuilder};
use futures_signals::{signal::SignalExt, signal_vec::SignalVecExt};
use shared::domain::{jig::codes::JigCodeSessionResponse, module::ModuleResponse};
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
        .child_signal(state.module.signal_cloned().map(clone!(state => move |module| {
            module.map(|module| {
                html!("div", {
                    .children_signal_vec(state.infos.signal_vec_cloned().map(clone!(state => move |info| {
                        state.render_session(info, &module)
                    })))
                })
            })
        })))

        // .child_signal(state.module_report_signal().map(|val| {
        //     val.map(|(module, report)| {
        //         html!("div", {
        //             .children(
        //                 report.items.iter().enumerate().map(|(i, round)| {
        //                     // html!("progress")
        //                     html!("div", {
        //                         .class("wrapper")
        //                         .child(html!("h4", {
        //                             .text("Round ")
        //                             .text(&i.to_string())
        //                         }))
        //                         .child(html!("div", {
        //                             .class("round-items")

        //                             .children(round.iter().map(|(id, item)| {
        //                                 html!("div", {
        //                                     .class("round-item")
        //                                     .child(html!("div", {
        //                                         .apply(|dom| {
        //                                             match &module.base.pairs[*id].0.card_content {
        //                                                 CardContent::Text(s) => {
        //                                                     dom.child(html!("p", {
        //                                                         .text(&s)
        //                                                     }))
        //                                                 },
        //                                                 CardContent::Image(Some(i)) => {
        //                                                     dom.child(html!("img-ji", {
        //                                                         // .prop("")
        //                                                     }))
        //                                                 },
        //                                                 CardContent::Image(None) => {
        //                                                     dom
        //                                                 }
        //                                             }
        //                                         })
        //                                     }))
        //                                     .child(html!("p", {
        //                                         .text("Tries ")
        //                                         .child(html!("strong", {
        //                                             .text(&(item.failed_tries + 1).to_string())
        //                                         }))
        //                                     }))
        //                                 })
        //                             }))
        //                         }))
        //                     })
        //                 })
        //             )
        //         })
        //     })
        // }))
    }
}

impl CodeSessions {
    fn render_session(
        self: &Rc<Self>,
        _info: JigCodeSessionResponse,
        _module: &ModuleResponse,
    ) -> Dom {
        html!("div", {
            .text("aaa")
        })
    }
    //     fn module_report_signal(
    //         self: &Rc<Self>,
    //     ) -> impl Signal<Item = Option<(matching::Content, JigPlaySessionModule)>> {
    //         map_ref! {
    //             let module = self.module.signal_ref(|module| {
    //                 // module.clone()
    //                 module.as_ref().map(|module| {
    //                     match &module.module.body {
    //                         shared::domain::module::ModuleBody::Matching(module) => module.content.clone().unwrap(),
    //                         _ => todo!(),
    //                     }
    //                 })
    //             }),
    //             let report = self.infos.signal_ref(|report| {
    //                 report.as_ref().map(|report| {
    //                     match &report.modules {
    //                         JigPlaySessionModule::Matching(report) => report.clone(),
    //                     }
    //                 })
    //             })
    //             => move {
    //                 match (module, report) {
    //                     (Some(module), Some(report)) => Some((module.clone(), report.clone())),
    //                     _ => None,
    //                 }
    //             }
    //         }
    //     }
}
