use dominator::{clone, Dom, html};
use std::rc::Rc;
use super::state::*;
use utils::prelude::*;

impl VerifyEmailPage {
    pub fn render(state: Rc<VerifyEmailPage>) -> Dom {
        html!("empty-fragment", {
            .future(clone!(state => async move {
                state.verify().await;
            }))
        })
    }
}
