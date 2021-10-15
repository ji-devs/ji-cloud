use super::state::*;
use dominator::{clone, html, Dom};
use std::rc::Rc;

impl VerifyEmailPage {
    pub fn render(state: Rc<VerifyEmailPage>) -> Dom {
        html!("empty-fragment", {
            .future(clone!(state => async move {
                state.verify().await;
            }))
        })
    }
}
