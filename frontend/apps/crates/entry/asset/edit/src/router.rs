use components::overlay::container::OverlayContainer;
use shared::domain::asset::AssetType;
use utils::routes::{AssetEditRoute, AssetRoute, Route};

use crate::{edit::AssetEditState, gallery::state::Gallery, studio::render_studio};
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
                AssetRoute::Edit(route) => match route {
                    AssetEditRoute::Jig(jig_id, _) => {
                        Some(AssetEditState::new(jig_id.into(), route).render())
                    }
                    AssetEditRoute::Resource(resource_id, _) => {
                        Some(AssetEditState::new(resource_id.into(), route).render())
                    }
                    AssetEditRoute::Course(course_id, _) => {
                        Some(AssetEditState::new(course_id.into(), route).render())
                    }
                    AssetEditRoute::ProDev(pro_dev_id, _) => {
                        Some(AssetEditState::new(pro_dev_id.into(), route).render())
                    }
                },
                AssetRoute::Studio => Some(render_studio()),
                AssetRoute::Play(_) => unimplemented!(), // Handled in player
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
