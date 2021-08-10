use std::rc::Rc;

use dominator::{Dom, clone, html};
use utils::events;

use super::state::State;

pub fn render(state: Rc<State>) -> Dom {
    html!("div", {
        .child(html!("input"))
        .child(html!("button", {
            .text("submit")
            .event(clone!(state => move |_: events::Click| {
                
            }))
        }))
    })
}
