use dominator::{html, Dom};
use shared::domain::{jig::codes::JigPlaySessionFindAnswer, module::body::find_answer};
pub fn render_find_answer(
    module: &find_answer::Content,
    session: &JigPlaySessionFindAnswer,
) -> Dom {
    html!("div", {
        .children(
            session.items.iter().enumerate().map(|(index, item)| {
                html!("div", {
                    .class("wrapper")
                    .child(
                        html!("div", {
                            .class("item")
                            .child(html!("div", {
                                .apply(|dom| {
                                    match &module.questions.get(index) {
                                        Some(question) => dom.text(&question.question_text),
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
                    )
                })
            })
        )
    })
}
