use dominator::{html, Dom, clone};
use crate::data::{raw, state::*};
use std::rc::Rc;
use utils::events;

pub struct ChooseDom {}
impl ChooseDom {
    pub fn render(state:Rc<State>) -> Dom {

        html!("choose-page", {
            .children(&mut [
                      html!("choose-card", {
                          .property("mode", "duplicate")
                          .event(clone!(state => move |evt:events::Click| {
                              state.change_mode(GameMode::Duplicate);
                          }))
                      }),
                      html!("choose-card", {
                          .property("mode", "words-images")
                          .event(clone!(state => move |evt:events::Click| {
                              state.change_mode(GameMode::WordsAndImages);
                          }))
                      }),
                      html!("choose-card", {
                          .property("mode", "begins")
                          .event(clone!(state => move |evt:events::Click| {
                              state.change_mode(GameMode::BeginsWith);
                          }))
                      }),
                      html!("choose-card", {
                          .property("mode", "lettering")
                      }),
            ])
        })
    }
}
