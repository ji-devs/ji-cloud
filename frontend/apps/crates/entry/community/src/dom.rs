use std::{collections::HashMap, rc::Rc};

use components::page_header::{PageHeader, PageHeaderConfig, PageLinks};
use dominator::{clone, html, with_node, Dom, EventOptions};
use futures_signals::signal::{Signal, SignalExt};
use shared::domain::user::{UserId, UserProfile};
use utils::{
    component::Component,
    events,
    init::analytics,
    prelude::{get_user_cloned, get_user_id},
    routes::{CommunityCirclesRoute, CommunityMembersRoute, CommunityRoute, Route, UserRoute},
};
use web_sys::HtmlInputElement;

use crate::{
    circle_details::CircleDetails, circle_list::CirclesList, courses_list::CoursesList,
    landing::CommunityLanding, member_details::MemberDetails, members_list::MembersList,
    search::CommunitySearch, state::Community,
};

const STR_SEARCH: &str = "Search Jigzi Community";

impl Community {
    pub fn render(self: &Rc<Self>) -> Dom {
        let state = self;
        analytics::event("Community Loaded", None);

        html!("community-main", {
            .children(&mut [
                PageHeader::new(PageHeaderConfig {
                    slot: Some("jigzi-header"),
                    active_page: Some(PageLinks::Community),
                    ..Default::default()
                }).render(),
                self.render_nav(),
                utils::link!(Route::Community(CommunityRoute::Landing), {
                    .prop("slot", "title")
                    .text("Community")
                }),
                html!("form", {
                    .prop("slot", "search-bar")
                    .event_with_options(
                        &EventOptions::preventable(),
                        clone!(state => move |e: events::Submit| {
                            e.prevent_default();
                            state.on_search_click();
                        })
                    )
                    .child(html!("community-search-bar", {
                        .children(&mut [
                            html!("input" => HtmlInputElement, {
                                .with_node!(elem => {
                                    .prop("slot", "search-input")
                                    .prop("type", "search")
                                    .prop("placeholder", STR_SEARCH)
                                    .prop_signal("value", state.q.signal_cloned())
                                    .event(clone!(state => move |_: events::Input| {
                                        let value = elem.value();
                                        state.q.set(value);
                                    }))
                                })
                            }),
                            html!("fa-button", {
                                .prop("slot", "search-button")
                                .prop("icon", "fa-solid fa-magnifying-glass")
                                .event(clone!(state => move |_: events::Click| {
                                    state.on_search_click();
                                }))
                            }),
                        ])
                    }))
                })
            ])
            .child_signal(self.dom_signal())
        })
    }

    fn dom_signal(self: &Rc<Self>) -> impl Signal<Item = Option<Dom>> {
        let state = self;
        Community::route_signal().map(clone!(state => move |route|{
            match route {
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
                    CommunityRoute::Courses => CoursesList::new().render(),
                }),
                _ => None,
            }
        }))
    }

    fn render_nav(self: &Rc<Self>) -> Dom {
        let user_id = get_user_id();
        html!("nav", {
            .prop("slot", "nav")
            .children(&mut [
                {
                    let route = match user_id {
                        Some(user_id) => {
                            Route::Community(CommunityRoute::Members(CommunityMembersRoute::Member(user_id)))
                        },
                        _ => {
                            Route::User(UserRoute::Login(Default::default()))
                        },
                    };
                    html!("community-nav-item", {
                        .child({
                            match get_user_cloned() {
                                Some(UserProfile { profile_image: Some(image_id), .. }) => {
                                    html!("profile-image", {
                                        .prop("imageId", &image_id.0.to_string())
                                    })
                                },
                                _ => {
                                    html!("fa-icon", {
                                        .prop("icon", "fa-thin fa-user-tie-hair")
                                    })
                                },
                            }
                        })
                        .prop("label", "My profile")
                        .prop("href", &route.to_string())
                        .prop_signal("active", Community::route_signal().map(move |route| {
                            matches!(route, Route::Community(CommunityRoute::Members(route)) if is_current_users_page(&user_id, &route))
                        }))
                        // only use local router if logged in, otherwise full redirect
                        .apply_if(user_id.is_some(), move |dom| utils::on_click_go_to_url!(dom, route))
                        .event(|_: events::Click| {
                            track_nav_item("My profile");
                        })
                    })
                },
                {
                    let route = Route::Community(CommunityRoute::Circles(CommunityCirclesRoute::List));
                    html!("community-nav-item", {
                        .prop("label", "Circles")
                        .prop("href", &route.to_string())
                        .prop_signal("active", Community::route_signal().map(|route| {
                            matches!(route, Route::Community(CommunityRoute::Circles(_)))
                        }))
                        .child(html!("fa-icon", {
                            .prop("icon", "fa-thin fa-circle-nodes")
                        }))
                        .event(|_: events::Click| {
                            track_nav_item("Circles");
                        })
                        .apply(move |dom| utils::on_click_go_to_url!(dom, route))
                    })
                },
                {
                    let route = Route::Community(CommunityRoute::Members(CommunityMembersRoute::List));
                    html!("community-nav-item", {
                        .prop("label", "Members")
                        .prop("href", &route.to_string())
                        .prop_signal("active", Community::route_signal().map(clone!(user_id => move |route| {
                            matches!(route, Route::Community(CommunityRoute::Members(route)) if !is_current_users_page(&user_id, &route))
                        })))
                        .child(html!("fa-icon", {
                            .prop("icon", "fa-thin fa-people-group")
                        }))
                        .event(|_: events::Click| {
                            track_nav_item("Members");
                        })
                        .apply(move |dom| utils::on_click_go_to_url!(dom, route))
                    })
                },
                {
                    let route = Route::Community(CommunityRoute::Courses);
                    html!("community-nav-item", {
                        .prop("label", "Courses")
                        .prop("href", &route.to_string())
                        .child(html!("fa-icon", {
                            .prop("icon", "fa-thin fa-clapperboard-play")
                        }))
                        .event(|_: events::Click| {
                            track_nav_item("Courses");
                        })
                        .apply(move |dom| utils::on_click_go_to_url!(dom, route))
                    })
                },
            ])
        })
    }
}

fn is_current_users_page(user_id: &Option<UserId>, member_route: &CommunityMembersRoute) -> bool {
    match (user_id, member_route) {
        (Some(user_id), CommunityMembersRoute::Member(active_user)) => user_id == active_user,
        _ => false,
    }
}

fn track_nav_item(item: &str) {
    let mut properties = HashMap::new();
    properties.insert("Item", item.to_owned());

    analytics::event("Community Nav Click", Some(properties));
}
