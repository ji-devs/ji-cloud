use dominator::{Dom, html};
use std::rc::Rc;
use super::state::*;

pub struct LocalePage {
    pub state: Rc<State>
}

impl LocalePage {
    pub fn render() -> Dom {
        let state = Rc::new(State::new());

        html!("div", {
            .text("hello world!")
        })
    }
}
