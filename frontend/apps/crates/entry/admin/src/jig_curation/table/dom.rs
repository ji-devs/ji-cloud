use super::state::*;
use components::module::_common::thumbnail::{ModuleThumbnail, ThumbnailFallback};
use convert_case::{Case, Casing};
use dominator::{clone, html, with_node, Dom};
use futures_signals::{
    map_ref,
    signal::{Signal, SignalExt},
    signal_vec::SignalVecExt,
};
use shared::domain::{
    asset::{DraftOrLive, OrderBy, PrivacyLevel},
    jig::JigRating,
    meta::{AffiliationId, AgeRangeId},
};
use std::rc::Rc;
use utils::{
    editable_asset::EditableJig, events, languages::Language, routes::AdminJigCurationRoute,
    unwrap::UnwrapJiExt,
};
use web_sys::HtmlSelectElement;

impl JigTable {
    pub fn render(self: Rc<Self>) -> Dom {
        let order_by_options = vec![OrderBy::PublishedAt, OrderBy::CreatedAt];

        let state = self;
        html!("admin-table-jig", {
            .child(html!("input-search", {
                .prop("slot", "search")
                .prop("placeholder", "Search...")
                .event(clone!(state => move |e: events::CustomSearch| {
                    state.search_jigs(e.query());
                }))
            }))
            .child(html!("table-order-by", {
                .prop("slot", "controls")
                .child(html!("input-select", {
                    .prop_signal("value", state.curation_state.order_by.signal().map(|order_by| {
                        format!("{}", order_by)
                    }))
                    .children(order_by_options.iter().map(|option| {
                        html!("input-select-option", {
                            .text(&format!("{}", option).to_string())
                            .prop_signal("selected", state.curation_state.order_by.signal().map(clone!(option => move |order_by| {
                                order_by == option
                            })))
                            .event(clone!(state, option => move |evt: events::CustomSelectedChange| {
                                if evt.selected() {
                                    state.curation_state.set_order_by(option);
                                }
                            }))
                        })
                    }))
                }))
            }))
            .child(html!("table-pagination-jig", {
                .prop("slot", "controls")
                .child(html!("fa-button", {
                    .prop("slot", "back")
                    .prop("title", "Previous")
                    .prop("icon", "fa-solid fa-chevron-left")
                    .prop_signal("disabled", state.curation_state.active_page.signal().map(|active_page| {
                        active_page == 0
                    }))
                    .event(clone!(state => move |_: events::Click| {
                        let active_page = state.curation_state.active_page.get();
                        state.curation_state.go_to_page(active_page - 1);
                    }))
                }))
                .child(html!("fa-button", {
                    .prop("slot", "next")
                    .prop("title", "Next")
                    .prop("icon", "fa-solid fa-chevron-right")
                    .prop_signal("disabled", map_ref! {
                        let total_pages = state.curation_state.total_pages.signal(),
                        let active_page = state.curation_state.active_page.signal() => {
                            match total_pages {
                                None => true,
                                Some(total_pages) => {
                                    // active_page is 0 indexed in the code side, so need to add 1 for display
                                    *active_page == total_pages - 1
                                }
                            }
                        }
                    })
                    .event(clone!(state => move |_: events::Click| {
                        let active_page = state.curation_state.active_page.get();
                        state.curation_state.go_to_page(active_page + 1);
                    }))
                }))
                .child_signal(state.curation_state.total_pages.signal().map(clone!(state => move |total_pages| {
                    total_pages.map(|total_pages| {
                        html!("input-select", {
                            .style("width", "150px")
                            .prop_signal("value", state.curation_state.active_page.signal().map(|active_page| {
                                format!("{}", active_page + 1)
                            }))
                            .children((0..total_pages).map(|page| {
                                html!("input-select-option", {
                                    .text(&format!("{}", page + 1).to_string())
                                    .prop_signal("selected", state.curation_state.active_page.signal().map(clone!(page => move |active_page| {
                                        page == active_page
                                    })))
                                    .event(clone!(state, page => move |evt: events::CustomSelectedChange| {
                                        if evt.selected() {
                                            state.curation_state.go_to_page(page);
                                        }
                                    }))
                                })
                            }))
                        })
                    })
                })))
            }))
            .children_signal_vec(state.curation_state.jigs.signal_vec_cloned().map(clone!(state => move |jig: Rc<EditableJig>| {
                let jig_id = jig.id;
                html!("admin-table-line", {
                    .child(html!("div", {
                        .style("display", "grid")
                        .style("grid-template-columns", "repeat(3, 100px)")
                        .style("align-items", "start")
                        .style("padding", "0")
                        .apply(|dom| {
                            let modules = jig.modules.lock_ref();
                            dom.children((0..3).filter_map(|i| {
                                modules.get(i).map(|module| {
                                    ModuleThumbnail::new(
                                        jig.id.into(),
                                        Some(module.clone()),
                                        ThumbnailFallback::Asset,
                                        DraftOrLive::Live,
                                    ).render(None)
                                })
                            }))
                        })
                    }))
                    .children(&mut [
                        html!("a", {
                            .text_signal(jig.display_name.signal_cloned())
                            .event(clone!(state => move |_: events::Click| {
                                let route = AdminJigCurationRoute::Jig(jig_id);
                                state.curation_state.navigate_to(route);
                            }))
                        }),

                        html!("input-checkbox", {
                            .prop_signal("checked", jig.premium.signal())
                            .event(clone!(state, jig => move |_evt: events::CustomToggle| {
                                jig.premium.set(!jig.premium.get());
                                state.curation_state.save_admin_data(&jig);
                            }))
                        }),

                        html!("span", {
                            .child(html!("fa-button", {
                                .prop("slot", "block")
                                .style_signal("color", jig.blocked.signal().map(|blocked| {
                                    match blocked {
                                        true => "red",
                                        false => "green",
                                    }
                                }))
                                .prop_signal("icon", jig.blocked.signal().map(|blocked| {
                                    match blocked {
                                        true => "fa-solid fa-eye-slash",
                                        false => "fa-solid fa-eye",
                                    }
                                }))
                                .prop_signal("title", jig.blocked.signal().map(|blocked| {
                                    match blocked {
                                        true => "Blocked",
                                        false => "Visible",
                                    }
                                }))
                                .event(clone!(state, jig => move |_: events::Click| {
                                    let mut blocked = jig.blocked.lock_mut();
                                    *blocked = !*blocked;

                                    state.curation_state.save_and_publish(&jig);
                                }))
                            }))
                        }),
                        html!("span", {
                            .text(&jig.author_name.clone().unwrap_or_default())
                        }),
                        html!("span", {
                            .text_signal(jig.plays.signal().map(|p| p.to_string()))
                        }),
                        html!("span", {
                            .text_signal(jig.likes.signal().map(|l| l.to_string()))
                        }),
                        html!("star-rating", {
                            .prop_signal("rating", jig.rating.signal().map(|rating| {
                                match rating {
                                    Some(rating) => rating as u8,
                                    None => 0,
                                }
                            }))
                            .event(clone!(state, jig => move |e: events::CustomRatingChange| {
                                let rating = e.rating();
                                let rating = rating.map(|rating| {
                                    JigRating::try_from(rating).unwrap_ji()
                                });
                                jig.rating.set(rating);
                                state.curation_state.save_and_publish(&jig);
                            }))
                        }),
                        html!("label", {
                            .child(html!("select" => HtmlSelectElement, {
                                .with_node!(select => {
                                    .prop_signal("value", jig.privacy_level.signal().map(|privacy_level| {
                                        privacy_level.as_str().to_case(Case::Title)
                                    }))
                                    .children(&mut [
                                        html!("option", {
                                            .text(&PrivacyLevel::Public.as_str().to_case(Case::Title))
                                        }),
                                        html!("option", {
                                            .text(&PrivacyLevel::Unlisted.as_str().to_case(Case::Title))
                                        }),
                                        // html!("option", {
                                        //     .text(&PrivacyLevel::Private.as_str().to_case(Case::Title))
                                        // }),
                                    ])
                                    .event(clone!(state, jig, select => move |_: events::Change| {
                                        let value = select.value().to_case(Case::Lower);
                                        let value = value.parse().unwrap_ji();
                                        jig.privacy_level.set(value);

                                        state.curation_state.save_and_publish(&jig);
                                    }))
                                })
                            }))
                        }),
                        html!("span", {
                            .text(&jig.created_at.format("%b %e, %Y").to_string())
                        }),
                        html!("span", {
                            .text_signal(jig.published_at.signal().map(|published_at| {
                                match published_at {
                                    Some(published_at) => published_at.format("%b %e, %Y").to_string(),
                                    None => "".to_string()
                                }
                            }))
                        }),
                        html!("span", {
                            .text_signal(jig.language.signal_cloned().map(|language| {
                                Language::code_to_display_name(&language)
                            }))
                        }),
                        html!("span", {
                            .style("display", "flex")
                            .style("flex-wrap", "wrap")
                            .style("column-gap", "16px")
                            .children_signal_vec(jig.age_ranges.signal_cloned().map(clone!(state => move |ages_hash| {
                                ages_hash.into_iter().map(|age_id| {
                                    html!("span", {
                                        .text_signal(state.age_label(age_id))
                                    })
                                }).collect()
                            })).to_signal_vec())
                        }),
                        html!("span", {
                            .style("display", "flex")
                            .style("flex-wrap", "wrap")
                            .style("column-gap", "16px")
                            .children_signal_vec(jig.affiliations.signal_cloned().map(clone!(state => move |affiliation_hash| {
                                affiliation_hash.into_iter().map(|affiliation_id| {
                                    html!("span", {
                                        .text_signal(state.affiliation_label(affiliation_id))
                                    })
                                }).collect()
                            })).to_signal_vec())
                        }),
                    ])
                })
            })))
        })
    }

    fn age_label(self: &Rc<Self>, age_id: AgeRangeId) -> impl Signal<Item = String> {
        self.curation_state.ages.signal_ref(move |ages| {
            let age = ages.iter().find(|age| age.id == age_id);
            match age {
                Some(age) => age.display_name.clone(),
                None => "-".to_string(),
            }
        })
    }

    fn affiliation_label(
        self: &Rc<Self>,
        affiliation_id: AffiliationId,
    ) -> impl Signal<Item = String> {
        self.curation_state
            .affiliations
            .signal_ref(move |affiliations| {
                let affiliation = affiliations
                    .iter()
                    .find(|affiliation| affiliation.id == affiliation_id);
                match affiliation {
                    Some(affiliation) => affiliation.display_name.clone(),
                    None => "-".to_string(),
                }
            })
    }
}
