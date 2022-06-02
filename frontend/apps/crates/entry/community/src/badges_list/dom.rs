use std::rc::Rc;

use dominator::{clone, html, Dom};
use futures_signals::signal_vec::SignalVecExt;
use shared::domain::badge::Badge;

use super::BadgesList;

impl BadgesList {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;
        state.load_badges();

        html!("div", {
            .text("badges")
            .children_signal_vec(state.badges.signal_vec_cloned().map(clone!(state => move |badge| {
                state.render_badge(badge)
            })))
        })
    }

    fn render_badge(self: &Rc<Self>, badge: Badge) -> Dom {
        html!("p", {
            .text(&badge.display_name)
        })
    }
}
