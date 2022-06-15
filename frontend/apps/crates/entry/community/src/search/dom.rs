use std::rc::Rc;

use dominator::{clone, html, Dom};
use futures_signals::signal_vec::SignalVecExt;
use shared::domain::user::public_user::PublicUser;

use super::CommunitySearch;

impl CommunitySearch {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;
        state.search();

        html!("div", {
            .children_signal_vec(state.members.signal_vec_cloned().map(clone!(state => move|member| {
                state.render_member(&member)
            })))
        })
    }

    fn render_member(self: &Rc<Self>, member: &PublicUser) -> Dom {
        html!("div", {
            .text(&member.given_name)
        })
    }
}
