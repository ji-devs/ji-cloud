use dominator::{html, Dom};
use shared::domain::{
    jig::codes::JigPlaySessionCardQuiz,
    module::body::{_groups::cards::CardContent, card_quiz},
};

pub fn render_card_quiz(module: &card_quiz::Content, session: &JigPlaySessionCardQuiz) -> Dom {
    html!("div", {
        .children(
            session.rounds.iter().enumerate().map(|(index, round)| {
                html!("div", {
                    .class("wrapper")
                    .child(html!("h4", {
                        .text("Round ")
                        .text(&(index + 1).to_string())
                    }))
                    .child(html!("div", {
                        .class("round-items")
                        .child(
                            html!("div", {
                                .class("round-item")
                                .child(html!("div", {
                                    .apply(|dom| {
                                        match &module.base.pairs.get(round.card_index) {
                                            Some(pair) => {
                                                match &pair.0.card_content {
                                                    CardContent::Text(s) => {
                                                        dom.child(html!("p", {
                                                            .text(&s)
                                                        }))
                                                    },
                                                    CardContent::Image(Some(image)) => {
                                                        dom.child(html!("img-ji", {
                                                            .prop("id", image.id.0.to_string())
                                                            .prop("lib", image.lib.to_str())
                                                        }))
                                                    },
                                                    CardContent::Image(None) => {
                                                        dom
                                                    }
                                                }
                                            },
                                            None => dom.text("?"),
                                        }

                                    })
                                }))
                                .child(html!("p", {
                                    .text("Tries ")
                                    .child(html!("strong", {
                                        .text(&(round.failed_tries + 1).to_string())
                                    }))
                                }))
                            })
                        )
                    }))
                })
            })
        )
    })
}
