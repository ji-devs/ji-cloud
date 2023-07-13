use std::rc::Rc;

use dominator::{html, Dom};
use futures_signals::signal::{from_future, SignalExt};
use shared::domain::asset::{Asset, DraftOrLive, PrivacyLevel};
use utils::ages::AgeRangeVecExt;
use utils::metadata::{get_age_ranges, get_resource_types};
use utils::routes::{CommunityMembersRoute, CommunityRoute, Route};

use crate::module::_common::thumbnail::{ModuleThumbnail, ThumbnailFallback};

#[derive(Clone, Default)]
pub struct AssetCardConfig<'a> {
    pub bottom_indicator: AssetCardBottomIndicator,
    pub slot: Option<&'a str>,
    pub dense: bool,
    pub menu: Option<Rc<dyn Fn() -> Dom>>,
}

#[derive(Default, Clone, Copy)]
pub enum AssetCardBottomIndicator {
    #[default]
    Hide,
    Author,
    Status,
}

pub fn render_asset_card(asset: &Asset, config: AssetCardConfig) -> Dom {
    // let liked = Mutable::new(false);
    html!("asset-card", {
        .prop("kind", asset.asset_type().as_str())
        .prop("dense", config.dense)
        .prop("name", asset.display_name())
        .prop("playedCount", asset.plays())
        .prop("likedCount", asset.likes())
        .prop("language", asset.language())
        .prop("premium", asset.premium())
        .prop("showBottomIndicator", !matches!(config.bottom_indicator, AssetCardBottomIndicator::Hide))
        .apply(|dom| {
            match config.slot {
                Some(slot) => dom.prop("slot", slot),
                None => dom,
            }
        })
        .apply(move |dom| {
            match config.menu {
                Some(menu) => dom.child(menu()),
                None => dom,
            }
        })
        .child(ModuleThumbnail::new(
            asset.id(),
            asset.cover().cloned(),
            ThumbnailFallback::Asset,
            DraftOrLive::Live
        ).render(Some("image")))
        // TODO: enable like
        // .apply_if(config.likeable, |dom| {
        // .apply_if(false, |dom| {
        //         dom.child(html!("fa-button", {
        //         .prop("slot", "like")
        //         .attr_signal("icon", liked.signal().map(|liked| {
        //             match liked {
        //                 true => "fa-solid fa-heart",
        //                 false => "fa-regular fa-heart",
        //             }
        //         }))
        //         .event(clone!(liked => move |_: events::Click| {
        //             let mut liked = liked.lock_mut();
        //             *liked = !*liked;
        //             todo!()
        //         }))
        //     }))
        // })
        .apply(|dom| {
            match asset {
                Asset::Jig(_) => dom,
                Asset::Playlist(playlist) => {
                    dom.child(html!("span", {
                        .prop("slot", "middle-indicator")
                        .text(&format!("{} Units", playlist.playlist_data.items.len()))
                    }))
                },
                Asset::Resource(resource) => {
                    match resource.resource_data.additional_resources.first() {
                        Some(resource) => {
                            let resource_type_id = resource.resource_type_id;
                            dom.child(html!("span", {
                                .prop("slot", "middle-indicator")
                                .text_signal(from_future(get_resource_types()).map(move |resource_types| {
                                    match resource_types {
                                        Some(resource_types) => {
                                            resource_types
                                                .iter()
                                                .find(move|t| t.id == resource_type_id)
                                                .map(|t| t.display_name.clone())
                                                .unwrap_or_default()
                                        },
                                        None => String::new(),
                                    }
                                }))
                            }))
                        },
                        None => dom
                    }
                },
                Asset::Course(course) => {
                    dom.child(html!("span", {
                        .prop("slot", "middle-indicator")
                        .text(&format!("{} Units", course.course_data.units.len()))
                    }))
                },

            }
        })
        // TODO: Hide age for Course
        .apply_if(((!asset.is_course()) && true), |dom| {
            let asset_age = asset.age_ranges().clone();
            dom.child_signal(from_future(get_age_ranges()).map(move |age_ranges| {
                age_ranges.map(|age_ranges| {
                    let (from, to) = age_ranges.range(&asset_age);
                    html!("age-range", {
                        .prop("slot", "ages")
                        .prop("icon", "entry/jig/play/sidebar/age.svg")
                        .prop("from", from)
                        .prop("to", to)
                    })
                })
            }))
        })
        .apply(|mut dom| {
            match config.bottom_indicator {
                AssetCardBottomIndicator::Hide => {
                    dom
                },
                AssetCardBottomIndicator::Author => {
                    if let (Some(name), Some(id)) = (asset.author_name(), asset.author_id()) {
                        let url = Route::Community(CommunityRoute::Members(CommunityMembersRoute::Member(*id))).to_string();
                        dom = dom.child(html!("a", {
                            .prop("slot", "bottom-indicator")
                            .prop("href", url)
                            .text(name)
                        }));
                    }
                    dom
                },
                AssetCardBottomIndicator::Status => {
                    if !asset.live_up_to_date() {
                        dom = dom.child(html!("span", {
                            .prop("slot", "bottom-indicator")
                            .child(html!("fa-icon", {
                                .prop("icon", "fa-regular fa-memo")
                            }))
                            .text("Draft")
                        }));
                    };

                    if !matches!(asset.privacy_level(), PrivacyLevel::Public) {
                        dom = dom.child(html!("span", {
                            .prop("slot", "bottom-indicator")
                            .child(html!("fa-icon", {
                                .prop("icon", "fa-regular fa-eye-slash")
                            }))
                            .text("Private")
                        }));
                    };

                    dom
                },
            }
        })
    })
}
