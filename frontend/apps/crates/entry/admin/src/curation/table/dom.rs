use crate::curation::EditableJig;

use super::state::*;
use components::module::_common::thumbnail::{ModuleThumbnail, ThumbnailFallback};
use dominator::{clone, html, Dom};
use futures_signals::{
    map_ref,
    signal::{Signal, SignalExt},
    signal_vec::SignalVecExt,
};
use shared::domain::{
    asset::{DraftOrLive, OrderBy},
    meta::{AffiliationId, AgeRangeId},
};
use std::rc::Rc;
use utils::{events, languages::Language, routes::AdminCurationRoute};

impl CurationTable {
    pub fn render(self: Rc<Self>) -> Dom {
        let order_by_options = vec![OrderBy::PublishedAt, OrderBy::CreatedAt];

        let state = self;
        html!("admin-curation-table", {
            .child(html!("input-search", {
                .property("slot", "search")
                .property("placeholder", "Search...")
                .event(clone!(state => move |e: events::CustomSearch| {
                    state.search_jigs(e.query());
                }))
            }))
            .child(html!("table-order-by", {
                .property("slot", "controls")
                .child(html!("input-select", {
                    .property_signal("value", state.curation_state.order_by.signal().map(|order_by| {
                        format!("{}", order_by)
                    }))
                    .children(order_by_options.iter().map(|option| {
                        html!("input-select-option", {
                            .text(&format!("{}", option).to_string())
                            .property_signal("selected", state.curation_state.order_by.signal().map(clone!(option => move |order_by| {
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
            .child(html!("table-pagination", {
                .property("slot", "controls")
                .child(html!("fa-button", {
                    .property("slot", "back")
                    .property("title", "Previous")
                    .property("icon", "fa-solid fa-chevron-left")
                    .property_signal("disabled", state.curation_state.active_page.signal().map(|active_page| {
                        active_page == 0
                    }))
                    .event(clone!(state => move |_: events::Click| {
                        let active_page = state.curation_state.active_page.get();
                        state.curation_state.go_to_page(active_page - 1);
                    }))
                }))
                .child(html!("fa-button", {
                    .property("slot", "next")
                    .property("title", "Next")
                    .property("icon", "fa-solid fa-chevron-right")
                    .property_signal("disabled", map_ref! {
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
                            .property_signal("value", state.curation_state.active_page.signal().map(|active_page| {
                                format!("{}", active_page + 1)
                            }))
                            .children((0..total_pages).map(|page| {
                                html!("input-select-option", {
                                    .text(&format!("{}", page + 1).to_string())
                                    .property_signal("selected", state.curation_state.active_page.signal().map(clone!(page => move |active_page| {
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
                html!("admin-curation-table-line", {
                    .child(html!("div", {
                        .style("display", "grid")
                        .style("grid-template-columns", "repeat(3, 100px)")
                        .style("align-items", "start")
                        .style("padding", "0")
                        .children((0..3).filter_map(|i| {
                            jig.modules.get(i).map(|module| {
                                ModuleThumbnail::render(
                                    Rc::new(ModuleThumbnail {
                                        asset_id: jig.id.into(),
                                        module: Some(module.clone()),
                                        fallback: ThumbnailFallback::Asset,
                                        draft_or_live: DraftOrLive::Live,
                                    }),
                                    None
                                )
                            })
                        }))
                    }))
                    .children(&mut [
                        html!("a", {
                            .text_signal(jig.display_name.signal_cloned())
                            .event(clone!(state => move |_: events::Click| {
                                let route = AdminCurationRoute::Jig(jig_id);
                                state.curation_state.navigate_to(route);
                            }))
                        }),

                        html!("span", {
                            .child(html!("fa-button", {
                                .property("slot", "block")
                                .style_signal("color", jig.blocked.signal().map(|blocked| {
                                    match blocked {
                                        true => "red",
                                        false => "green",
                                    }
                                }))
                                .property_signal("icon", jig.blocked.signal().map(|blocked| {
                                    match blocked {
                                        true => "fa-solid fa-eye-slash",
                                        false => "fa-solid fa-eye",
                                    }
                                }))
                                .property_signal("title", jig.blocked.signal().map(|blocked| {
                                    match blocked {
                                        true => "Blocked",
                                        false => "Visible",
                                    }
                                }))
                                .event(clone!(jig => move |_: events::Click| {
                                    let mut blocked = jig.blocked.lock_mut();
                                    *blocked = !*blocked;

                                    jig.save_and_publish();
                                }))
                            }))
                        }),
                        html!("span", {
                            .text(&jig.author_name)
                        }),
                        html!("star-rating", {
                            .property_signal("rating", jig.rating.signal().map(|rating| {
                                match rating {
                                    Some(rating) => rating as u8,
                                    None => 0,
                                }
                            }))
                        }),
                        html!("span", {
                            .text(&jig.created_at.format("%b %e, %Y").to_string())
                        }),
                        html!("span", {
                            .text(&match jig.published_at {
                                Some(published_at) => published_at.format("%b %e, %Y").to_string(),
                                None => "".to_string()
                            })
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
