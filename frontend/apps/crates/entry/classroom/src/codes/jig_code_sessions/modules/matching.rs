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
                                        match module.base.pairs.get(*id) {
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
