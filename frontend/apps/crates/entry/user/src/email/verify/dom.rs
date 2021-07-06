use dominator::{clone, Dom, html};
use std::rc::Rc;
use super::state::*;
use utils::prelude::*;

impl VerifyEmailPage {
    pub fn render(state: Rc<VerifyEmailPage>) -> Dom {
        html!("h1", {
            .text(&format!("TODO: verify {}", state.token))
        })
    }
}
