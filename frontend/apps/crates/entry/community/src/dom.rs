use std::rc::Rc;

use components::{
    overlay::container::OverlayContainer,
    page_header::{self, state::PageLinks},
};
use dominator::{html, Dom};
use futures_signals::signal::{Signal, SignalExt};
use shared::domain::user::UserProfile;
use utils::{
    prelude::get_user,
    routes::{CommunityBadgesRoute, CommunityMembersRoute, CommunityRoute, Route, UserRoute},
};

use crate::{
    badge_details::BadgeDetails, badges_list::BadgesList, landing::CommunityLanding,
    member_details::MemberDetails, members_list::MembersList, state::Community,
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
        Community::route_signal().map(|route| match route {
            Route::Community(route) => Some(match route {
                CommunityRoute::Landing => CommunityLanding::new().render(),
                CommunityRoute::Members(route) => match route {
                    CommunityMembersRoute::List => MembersList::new().render(),
                    CommunityMembersRoute::Member(member_id) => {
                        MemberDetails::new(member_id).render()
                    }
                },
                CommunityRoute::Badges(route) => match route {
                    CommunityBadgesRoute::List => BadgesList::new().render(),
                    CommunityBadgesRoute::Badge(badge_id) => BadgeDetails::new(badge_id).render(),
                },
            }),
            _ => None,
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
                    .property_signal("active", Community::route_signal().map(|route| {
                        matches!(route, Route::Community(CommunityRoute::Landing))
                    }))
                    .child({
                        match get_user() {
                            Some(UserProfile { profile_image: Some(image_id), .. }) => {
                                html!("profile-image", {
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
                    .apply(move |dom| dominator::on_click_go_to_url!(dom, {
                        match get_user() {
                            Some(user) => {
                                Route::Community(CommunityRoute::Members(CommunityMembersRoute::Member(user.id))).to_string()
                            },
                            _ => {
                                Route::User(UserRoute::Login(Default::default())).to_string()
                            },
                        }
                    }))
                }),
                html!("community-nav-item", {
                    .property("label", "Badges")
                    .property_signal("active", Community::route_signal().map(|route| {
                        matches!(route, Route::Community(CommunityRoute::Badges(_)))
                    }))
                    .child(html!("fa-icon", {
                        .property("icon", "fa-thin fa-circle-nodes")
                    }))
                    .apply(move |dom| dominator::on_click_go_to_url!(dom, {
                        Route::Community(CommunityRoute::Badges(CommunityBadgesRoute::List)).to_string()
                    }))
                }),
                html!("community-nav-item", {
                    .property("label", "Members")
                    .property_signal("active", Community::route_signal().map(|route| {
                        matches!(route, Route::Community(CommunityRoute::Members(_)))
                    }))
                    .child(html!("fa-icon", {
                        .property("icon", "fa-thin fa-people-group")
                    }))
                    .apply(move |dom| dominator::on_click_go_to_url!(dom, {
                        Route::Community(CommunityRoute::Members(CommunityMembersRoute::List)).to_string()
                    }))
                }),
                html!("community-nav-item", {
                    .property("label", "ProDev")
                    .child(html!("fa-icon", {
                        .property("icon", "fa-thin fa-clapperboard-play")
                    }))
                }),
            ])
        })
    }
}
