use components::stickers::embed::types::{ParseUrlExt, PartialQuizletEmbed};
use dominator::{clone, html, with_node, Dom};
use shared::domain::module::body::_groups::design::QuizletId;
use std::rc::Rc;
use utils::events;
use web_sys::{HtmlElement, HtmlInputElement};

use crate::base::sidebar::step_2::actions;

use super::super::state::Step2;

pub fn render_quizlet(state: &Rc<Step2>, quizlet: &Rc<PartialQuizletEmbed>) -> Dom {
    html!("div", {
        .child(html!("input-wrapper" => HtmlElement, {
            .with_node!(wrapper => {
                .prop("slot", "input")
                .prop("label", "Add a Quizlet link")
                .child(html!("input" => HtmlInputElement, {
                    .prop("value", {
                        // not using a signal because the value can be invalid but should still show up
                        match quizlet.url.get_cloned() {
                            Some(url) => url.0.clone(),
                            None => String::new(),
                        }
                    })
                    .with_node!(input => {
                        .event(clone!(state, quizlet => move |_: events::Input| {
                            match QuizletId::try_parse(input.value()) {
                                Err(_) => {
                                    actions::set_error(&wrapper, true);
                                    quizlet.url.set(None);
                                }
                                Ok(quizlet_url) => {
                                    actions::set_error(&wrapper, false);
                                    quizlet.url.set(Some(quizlet_url));
                                },
                            };
                            state.on_embed_value_change();
                        }))
                    })
                }))
            })
        }))
    })
}
