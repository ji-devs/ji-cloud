use super::state::*;
use components::module::_common::thumbnail::{ModuleThumbnail, ThumbnailFallback};
use convert_case::{Case, Casing};
use dominator::{clone, html, with_node, Dom};
use futures_signals::{map_ref, signal::SignalExt, signal_vec::SignalVecExt};
use shared::domain::course::OrderBy;
use shared::domain::{
    asset::{DraftOrLive, PrivacyLevel},
    course::CourseRating,
};
use std::rc::Rc;
use utils::{
    editable_asset::EditableCourse, events, languages::Language, routes::AdminCourseCurationRoute,
    unwrap::UnwrapJiExt,
};
use web_sys::HtmlSelectElement;

impl CourseTable {
    pub fn render(self: Rc<Self>) -> Dom {
        let order_by_options = vec![OrderBy::PlayCount];

        let state = self;
        html!("admin-table-course", {
            .child(html!("input-search", {
                .prop("slot", "search")
                .prop("placeholder", "Search...")
                .event(clone!(state => move |e: events::CustomSearch| {
                    state.search_courses(e.query());
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
            .child(html!("table-pagination-course", {
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
            .children_signal_vec(state.curation_state.courses.signal_vec_cloned().map(clone!(state => move |course: Rc<EditableCourse>| {
                let course_id = course.id;
                html!("admin-table-line", {
                    .child(html!("div", {
                        .style("display", "grid")
                        .style("align-items", "start")
                        .child_signal(course.cover.signal_cloned().map(clone!(course => move|cover| {
                            cover.map(|cover| {
                                ModuleThumbnail::new(
                                    course.id.into(),
                                    Some(cover.clone()),
                                    ThumbnailFallback::Asset,
                                    DraftOrLive::Live,
                                ).render(None)
                            })
                        })))
                    }))
                    .children(&mut [
                        html!("a", {
                            .text_signal(course.display_name.signal_cloned())
                            .event(clone!(state => move |_: events::Click| {
                                let route = AdminCourseCurationRoute::Course(course_id);
                                state.curation_state.navigate_to(route);
                            }))
                        }),

                        html!("input-checkbox", {
                            .prop_signal("checked", course.premium.signal())
                            .event(clone!(state, course => move |_evt: events::CustomToggle| {
                                course.premium.set(!course.premium.get());
                                state.curation_state.save_admin_data(&course);
                            }))
                        }),

                        html!("span", {
                            .child(html!("fa-button", {
                                .prop("slot", "block")
                                .style_signal("color", course.blocked.signal().map(|blocked| {
                                    match blocked {
                                        true => "red",
                                        false => "green",
                                    }
                                }))
                                .prop_signal("icon", course.blocked.signal().map(|blocked| {
                                    match blocked {
                                        true => "fa-solid fa-eye-slash",
                                        false => "fa-solid fa-eye",
                                    }
                                }))
                                .prop_signal("title", course.blocked.signal().map(|blocked| {
                                    match blocked {
                                        true => "Blocked",
                                        false => "Visible",
                                    }
                                }))
                                .event(clone!(state, course => move |_: events::Click| {
                                    let mut blocked = course.blocked.lock_mut();
                                    *blocked = !*blocked;

                                    state.curation_state.save_and_publish(&course);
                                }))
                            }))
                        }),
                        html!("star-rating", {
                            .prop_signal("rating", course.rating.signal().map(|rating| {
                                match rating {
                                    Some(rating) => rating as u8,
                                    None => 0,
                                }
                            }))
                            .event(clone!(state, course => move |e: events::CustomNumber| {
                                let rating = e.number();
                                let rating = rating.map(|rating| {
                                    CourseRating::try_from(rating as u8).unwrap_ji()
                                });
                                course.rating.set(rating);
                                state.curation_state.save_and_publish(&course);
                            }))
                        }),
                        html!("label", {
                            .child(html!("select" => HtmlSelectElement, {
                                .with_node!(select => {
                                    .prop_signal("value", course.privacy_level.signal().map(|privacy_level| {
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
                                    .event(clone!(state, course, select => move |_: events::Change| {
                                        let value = select.value().to_case(Case::Lower);
                                        let value = value.parse().unwrap_ji();
                                        course.privacy_level.set(value);

                                        state.curation_state.save_and_publish(&course);
                                    }))
                                })
                            }))
                        }),
                        html!("span", {
                            .text_signal(course.published_at.signal_cloned().map(|published_at| {
                                match published_at {
                                    Some(published_at) => published_at.format("%b %e, %Y").to_string(),
                                    None => "".to_string()
                                }
                            }))
                        }),
                        html!("span", {
                            .text_signal(course.language.signal_cloned().map(|language| {
                                Language::code_to_display_name(&language)
                            }))
                        }),
                    ])
                })
            })))
        })
    }
}
