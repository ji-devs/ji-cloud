use std::rc::Rc;

use components::{
    overlay::container::OverlayContainer,
    page_header::{self, state::PageLinks},
};
use dominator::{html, Dom};
use futures_signals::signal::Signal;
use shared::domain::user::UserProfile;
use utils::{
    prelude::get_user,
    routes::{CommunityBadgesRoute, CommunityMembersRoute, CommunityRoute, Route},
};

use crate::{
    badge_details::BadgeDetails, badges_list::BadgesList, member_details::MemberDetails,
    members_list::MembersList, profile::CommunityProfile, state::Community,
};

impl Community {
    pub fn render(self: &Rc<Self>) -> Dom {
        html!("main", {
            // main header
            .child(page_header::dom::render(
                Rc::new(page_header::state::State::new()),
                None,
                Some(PageLinks::Community),
                true
            ))
            // community header
            .child(self.render_header())
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
                        CommunityBadgesRoute::Badge(badge_id) => {
                            BadgeDetails::new(badge_id).render()
                        }
                    },
                }),
                _ => None,
            }
        })
    }

    fn render_header(self: &Rc<Self>) -> Dom {
        html!("community-header", {
            .child(self.render_nav())
        })
    }

    fn render_nav(self: &Rc<Self>) -> Dom {
        html!("nav", {
            .property("slot", "nav")
            .children(&mut [
                html!("community-nav-item", {
                    .child({
                        match get_user() {
                            Some(UserProfile { profile_image: Some(image_id), .. }) => {
                                html!("profile-image", {
                                    .property("slot", "profile-image")
                                    .property("imageId", &image_id.0.to_string())
                                })
                            },
                            _ => {
                                html!("fa-icon", {
                                    .property("icon", "fa-thin fa-user-tie-hair")
                                })
                            },
                        }
                    })
                    .property("label", "My profile")
                }),
                html!("community-nav-item", {
                    .property("href", "/community/badges")
                    .property("label", "Badges")
                    .child(html!("fa-icon", {
                        .property("icon", "fa-thin fa-circle-nodes")
                    }))
                }),
                html!("community-nav-item", {
                    .property("href", "/community/members")
                    .property("label", "Members")
                    .child(html!("fa-icon", {
                        .property("icon", "fa-thin fa-circle-nodes")
                    }))
                }),
                html!("community-nav-item", {
                    .property("label", "ProDev")
                    .child(html!("fa-icon", {
                        .property("icon", "fa-thin fa-circle-nodes")
                    }))
                }),
            ])
        })
    }
}
