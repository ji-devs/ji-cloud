use std::rc::Rc;

use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use shared::domain::{badge::Badge, user::public_user::PublicUser};

use super::CommunityLanding;

impl CommunityLanding {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;
        state.load_data();

        html!("div", {
            .children_signal_vec(state.top_members.signal_ref(clone!(state => move |top_members| {
                match top_members {
                    None => vec![html!("progress")],
                    Some(top_members) => {
                        top_members.iter().map(|member| {
                            state.render_member(member)
                        }).collect()
                    },
                }
            })).to_signal_vec())
            .children_signal_vec(state.top_badges.signal_ref(clone!(state => move |top_badges| {
                match top_badges {
                    None => vec![html!("progress")],
                    Some(top_badges) => {
                        top_badges.iter().map(|badge| {
                            state.render_badge(badge)
                        }).collect()
                    },
                }
            })).to_signal_vec())
        })
    }

    fn render_member(self: &Rc<Self>, member: &PublicUser) -> Dom {
        html!("div", {
            .text(&member.given_name)
        })
    }

    fn render_badge(self: &Rc<Self>, badge: &Badge) -> Dom {
        html!("div", {
            .text(&badge.display_name)
        })
    }
}
