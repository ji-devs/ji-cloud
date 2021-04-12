use dominator::{html, Dom, clone};
use crate::data::{raw, state::*};
use std::rc::Rc;
use utils::events;

pub struct ChooseDom {}
impl ChooseDom {
    pub fn render(state:Rc<State>) -> Dom {

        html!("choose-page", {
            .children(
                [
                    GameMode::Duplicate,
                    GameMode::WordsAndImages,
                    GameMode::BeginsWith,
                    GameMode::Lettering,
                    GameMode::Riddles,
                    GameMode::Opposites,
                    GameMode::Synonymns,
                    GameMode::Translate
                ]
                .into_iter()
                .map(|mode| {
                    let mode = *mode;
                    html!("choose-card", {
                        .property("mode", mode.as_str())
                        .event(clone!(state => move |evt:events::Click| {
                            state.change_mode(mode);
                        }))
                    })
                })
            )
        })
    }
}
