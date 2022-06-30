use std::rc::Rc;

use components::{
    overlay::container::OverlayContainer,
    page_header::{self, state::PageLinks},
};
use dominator::{clone, html, with_node, Dom};
use futures_signals::signal::{Signal, SignalExt};
use shared::domain::user::UserProfile;
use utils::{
    events,
    prelude::get_user,
    routes::{
        CommunityCirclesRoute, CommunityMembersRoute, CommunityRoute, CommunitySearchQuery, Route,
        UserRoute,
    },
};
use web_sys::HtmlInputElement;

use crate::{
    circle_details::CircleDetails, circle_list::CirclesList, landing::CommunityLanding,
    member_details::MemberDetails, members_list::MembersList, search::CommunitySearch,
    state::Community,
};

const STR_SEARCH: &str = "Hebrew teachers";

impl Community {
    pub fn render(self: &Rc<Self>) -> Dom {
        let state = self;
        state.load_data();

        html!("community-main", {
            .children(&mut [
                page_header::dom::render(
                    Rc::new(page_header::state::State::new()),
                    Some("jigzi-header"),
                    Some(PageLinks::Community),
                    true
                ),
                self.render_nav(),
                html!("input" => HtmlInputElement, {
                    .with_node!(elem => {
                        .property("slot", "search-input")
                        .property("type", "search")
                        .property("placeholder", STR_SEARCH)
                        .property_signal("value", state.q.signal_cloned())
                        .event(clone!(state => move |_: events::Input| {
                            let value = elem.value();
                            state.q.set(value);
                        }))
                    })
                }),
                html!("fa-button", {
                    .property("slot", "search-button")
                    .property("icon", "fa-solid fa-magnifying-glass")
                    .event(clone!(state => move |_: events::Click| {
                        let query = CommunitySearchQuery {
                            q: state.q.get_cloned(),
                        };
                        dominator::routing::go_to_url(
                            &Route::Community(CommunityRoute::Search(Box::new(query))).to_string()
                        );
                    }))
                }),
            ])
            .child_signal(self.dom_signal())
            .child(OverlayContainer::new().render(None))
        })
    }

    fn dom_signal(self: &Rc<Self>) -> impl Signal<Item = Option<Dom>> {
        let state = self;
        Community::route_signal().map(clone!(state => move |route| match route {
            Route::Community(route) => Some(match route {
                CommunityRoute::Landing => CommunityLanding::new().render(),
                CommunityRoute::Search(search) => {
                    state.q.set(search.q.clone());
                    CommunitySearch::new(*search).render()
                },
                CommunityRoute::Members(route) => match route {
                    CommunityMembersRoute::List => MembersList::new().render(),
                    CommunityMembersRoute::Member(member_id) => {
                        MemberDetails::new(
                            Rc::clone(&state),
                            member_id
                        ).render()
                    }
                },
                CommunityRoute::Circles(route) => match route {
                    CommunityCirclesRoute::List => CirclesList::new().render(),
                    CommunityCirclesRoute::Circle(circle_id) => {
                        CircleDetails::new(
                            Rc::clone(&state),
                            circle_id
                        ).render()
                    },
                },
            }),
            _ => None,
        }))
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
                    .property("label", "Circles")
                    .property_signal("active", Community::route_signal().map(|route| {
                        matches!(route, Route::Community(CommunityRoute::Circles(_)))
                    }))
                    .child(html!("fa-icon", {
                        .property("icon", "fa-thin fa-circle-nodes")
                    }))
                    .apply(move |dom| dominator::on_click_go_to_url!(dom, {
                        Route::Community(CommunityRoute::Circles(CommunityCirclesRoute::List)).to_string()
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
