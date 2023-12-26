use std::rc::Rc;

use dominator::{html, DomBuilder};
use shared::domain::module::body::_groups::cards::CardContent;
use utils::component::Component;
use web_sys::ShadowRoot;

use super::Matching;

impl Component<Matching> for Rc<Matching> {
    fn styles() -> &'static str {
        include_str!("./styles.css")
    }

    fn dom(&self, dom: DomBuilder<ShadowRoot>) -> DomBuilder<ShadowRoot> {
        let state = self;

        dom.child(html!("div", {
            .children(
                self.session.rounds.iter().enumerate().map(|(i, round)| {
                    html!("div", {
                        .class("wrapper")
                        .child(html!("h4", {
                            .text("Round ")
                            .text(&i.to_string())
                        }))
                        .child(html!("div", {
                            .class("round-items")

                            .children(round.iter().map(|(id, item)| {
                                html!("div", {
                                    .class("round-item")
                                    .child(html!("div", {
                                        .apply(|dom| {
                                            match &state.module.base.pairs[*id].0.card_content {
                                                CardContent::Text(s) => {
                                                    dom.child(html!("p", {
                                                        .text(&s)
                                                    }))
                                                },
                                                CardContent::Image(Some(_i)) => {
                                                    dom.child(html!("img-ji", {
                                                        // .prop("")
                                                    }))
                                                },
                                                CardContent::Image(None) => {
                                                    dom
                                                }
                                            }
                                        })
                                    }))
                                    .child(html!("p", {
                                        .text("Tries ")
                                        .child(html!("strong", {
                                            .text(&(item.failed_tries + 1).to_string())
                                        }))
                                    }))
                                })
                            }))
                        }))
                    })
                })
            )
        }))
    }
}
