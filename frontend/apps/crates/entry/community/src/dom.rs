use std::rc::Rc;

use components::overlay::container::OverlayContainer;
use dominator::{html, Dom};
use futures_signals::signal::Signal;
use utils::routes::{Route, CommunityRoute, CommunityMembersRoute, CommunityBadgesRoute};

use crate::{state::Community, profile::CommunityProfile, members_list::MembersList, badges_list::BadgesList, badge_details::BadgeDetails, member_details::MemberDetails};

impl Community {
    pub fn render(self: &Rc<Self>) -> Dom {
        html!("main", {
            .child_signal(Self::dom_signal())
            .child(OverlayContainer::new().render(None))
        })
    }

    fn dom_signal() -> impl Signal<Item = Option<Dom>> {
        dominator::routing::url().signal_ref(|url| {
            let route = Route::from_url(url);
            match route {
                Route::Community(route) => Some(match route {
                    CommunityRoute::Landing => html!("div", {
                        .text("community")
                    }),
                    CommunityRoute::Profile => CommunityProfile::new().render(),
                    CommunityRoute::Members(route) => match route {
                        CommunityMembersRoute::List => MembersList::new().render(),
                        CommunityMembersRoute::Member(_member_id) => MemberDetails::new().render(),
                    },
                    CommunityRoute::Badges(route) => match route {
                        CommunityBadgesRoute::List => BadgesList::new().render(),
                        CommunityBadgesRoute::Badge(badge_id) => BadgeDetails::new(badge_id).render(),
                    },
                }),
                _ => None,
            }
        })
    }
}
