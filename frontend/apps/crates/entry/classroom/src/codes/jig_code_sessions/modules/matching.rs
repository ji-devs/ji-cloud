use dominator::{html, Dom};
use shared::domain::{
    jig::codes::JigPlaySessionMatching,
    module::body::{_groups::cards::CardContent, matching},
};

pub fn render_matching(module: &matching::Content, session: &JigPlaySessionMatching) -> Dom {
    html!("div", {
        .children(
            session.rounds.iter().enumerate().map(|(i, round)| {
                html!("div", {
                    .class("wrapper")
                    .child(html!("h4", {
                        .text("Round ")
                        .text(&(i + 1).to_string())
                    }))
                    .child(html!("div", {
                        .class("round-items")
                        .children(round.iter().map(|(id, item)| {
                            html!("div", {
                                .class("round-item")
                                .child(html!("div", {
                                    .apply(|dom| {
                                        match &module.base.pairs[*id].0.card_content {
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
    })
}

pub fn get_matching_count(session: &JigPlaySessionMatching) -> String {
    let mut total = 0;
    let mut earned = 0;
    for round in &session.rounds {
        for (_, card) in round {
            total += 2;
            earned += match card.failed_tries {
                0 => 2,
                1 => 1,
                _ => 0,
            };
        }
    }
    format!("{earned}/{total}")
}
