use components::overlay::container::OverlayContainer;
use utils::routes::*;

use dominator::{html, Dom};
use futures_signals::signal::{Signal, SignalExt};

use crate::{profile::CommunityProfile, members::CommunityMembers, badges_list::BadgesList, badge_details::BadgeDetails};

pub struct Router {}

impl Router {
    pub fn new() -> Self {
        Self {}
    }

    fn signal() -> impl Signal<Item = Route> {
        dominator::routing::url().signal_ref(|url| Route::from_url(url))
    }

    fn dom_signal() -> impl Signal<Item = Option<Dom>> {
        Self::signal().map(|route| match route {
            Route::Community(route) => Some(match route {
                CommunityRoute::Landing => html!("div", {
                    .text("community")
                }),
                CommunityRoute::Profile => {
                    CommunityProfile::new().render()
                },
                CommunityRoute::Members(_) => {
                    CommunityMembers::new().render()
                },
                CommunityRoute::Badges(route) => {
                    match route {
                        CommunityBadgesRoute::List => BadgesList::new().render(),
                        CommunityBadgesRoute::Badge(badge_id) => BadgeDetails::new(badge_id).render(),
                    }
                },
            }),
            _ => None,
        })
    }

    pub fn render(&self) -> Dom {
        html!("main", {
            .child_signal(Self::dom_signal())
            .child(OverlayContainer::new().render(None))
        })
    }
}
