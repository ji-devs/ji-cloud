use components::overlay::container::OverlayContainer;
use shared::domain::{asset::AssetType, jig::JigFocus};
use utils::routes::{AssetEditRoute, AssetRoute, Route};

use crate::{edit::dom::EditPage, gallery::state::Gallery};
use dominator::{html, Dom};
use futures_signals::signal::{Signal, SignalExt};

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
            Route::Asset(route) => match route {
                AssetRoute::JigGallery => Some(Gallery::new(AssetType::Jig).render()),
                AssetRoute::ResourceGallery => Some(Gallery::new(AssetType::Resource).render()),
                AssetRoute::CourseGallery => Some(Gallery::new(AssetType::Course).render()),
                AssetRoute::Edit(AssetEditRoute::Jig(jig_id, jig_focus, route)) => {
                    match jig_focus {
                        JigFocus::Modules => {
                            Some(EditPage::render(jig_id.into(), JigFocus::Modules, route))
                        },
                        JigFocus::Resources => {
                            Some(EditPage::render(jig_id.into(), JigFocus::Resources, route))
                        },
                    }
                }
                _ => None,
            },
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
