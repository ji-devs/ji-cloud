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
                                state.index.data.set(Some(raw::GameData::new_duplicate()));
                            }))
                      }),
                      html!("choose-card", {
                          .property("mode", "words-images")
                      }),
                      html!("choose-card", {
                          .property("mode", "begins")
                      }),
                      html!("choose-card", {
                          .property("mode", "lettering")
                      }),
            ])
        })
    }
}
