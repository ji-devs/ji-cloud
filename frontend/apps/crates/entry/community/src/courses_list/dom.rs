use std::rc::Rc;

use components::{
    asset_card::{render_asset_card, AssetCardBottomIndicator, AssetCardConfig},
    player_popup::{PlayerPopup, PreviewPopupCallbacks},
};
use dominator::{class, clone, html, pseudo, with_node, DomBuilder};
use futures_signals::{map_ref, signal::SignalExt};
use utils::{component::Component, events};
use web_sys::{HtmlInputElement, ShadowRoot};

use super::CoursesList;

impl Component<CoursesList> for Rc<CoursesList> {
    fn styles() -> &'static str {
        include_str!("./styles.css")
    }

    fn dom(&self, dom: DomBuilder<ShadowRoot>) -> DomBuilder<ShadowRoot> {
        let state = self;
        state.load_courses();

        dom.child(html!("div", {
            .class("header")
            .child(html!("h1", {
                .text("Courses")
            }))
            .child(html!("span", {
                .class("courses-count")
                .text_signal(self.total_course_count.signal().map(|t| {
                    t.map(|t| t.to_string()).unwrap_or_default()
                }))
            }))
        }))
        .children_signal_vec(
            state
                .courses
                .signal_ref(clone!(state => move |courses| {
                    match courses {
                        None => {
                            vec![html!("progress", {
                                .prop("slot", "items")
                            })]
                        }
                        Some(courses) => {
                            courses
                                .iter()
                                .map(clone!(state => move |course| {
                                    let courses_id = course.id;

                                    html!("div", {
                                        .style("cursor", "pointer")
                                        .child(render_asset_card(
                                            &course.clone().into(),
                                            AssetCardConfig {
                                                bottom_indicator: AssetCardBottomIndicator::Author,
                                                dense: true,
                                                ..Default::default()
                                            }
                                        ))
                                        .event(clone!(state => move |_: events::Click| {
                                            state.play_course.set(Some(courses_id));
                                        }))
                                    })
                                }))
                                .collect()
                        }
                    }
                }))
                .to_signal_vec(),
        )
        .child(html!("community-pagination", {
            .prop("slot", "sort-header")
            .prop_signal("total", state.total_pages.signal())
            .children(&mut [
                html!("fa-button", {
                    .prop("slot", "back")
                    .prop("icon", "fa-solid fa-angle-left")
                    .prop_signal("disabled", state.active_page.signal().map(|active_page| {
                        active_page <= 1
                    }))
                    .event(clone!(state => move |_: events::Click| {
                        let active_page = state.active_page.get();
                        if active_page > 1 {
                            state.active_page.set(active_page - 1);
                            state.load_courses();
                        };
                    }))
                }),
                html!("input" => HtmlInputElement, {
                    .with_node!(elem => {
                        .class(class! {
                            .pseudo!("::-webkit-outer-spin-button", {
                                .style("-webkit-appearance", "none")
                                .style("margin", "0")
                            })
                            .pseudo!("::-webkit-inner-spin-button", {
                                .style("-webkit-appearance", "none")
                                .style("margin", "0")
                            })
                        })
                        .prop("slot", "active-page")
                        .prop("type", "number")
                        .prop("min", 1)
                        .prop_signal("max", state.total_pages.signal())
                        .prop_signal("value", state.active_page.signal().map(|active_page| {
                            active_page.to_string()
                        }))
                        .event(clone!(state, elem => move |_: events::Input| {
                            let value = elem.value();
                            if let Ok(num) = value.parse::<u32>() {
                                if num <= state.total_pages.get() {
                                    state.active_page.set(num);
                                    state.load_courses();
                                }
                            };
                        }))
                    })
                }),
                html!("fa-button", {
                    .prop("slot", "forward")
                    .prop("icon", "fa-solid fa-angle-right")
                    .prop_signal("disabled", map_ref! {
                        let active_page = state.active_page.signal(),
                        let total_pages = state.total_pages.signal() => {
                            active_page >= total_pages
                        }
                    })
                    .event(clone!(state => move |_: events::Click| {
                        state.active_page.replace_with(|active_page| {
                            *active_page + 1
                        });
                        state.load_courses();
                    }))
                }),
            ])
        }))
        .child_signal(state.play_course.signal().map(clone!(state => move |play_course| {
            play_course.map(|course_id| {
                PlayerPopup::new_default_player_options(
                    course_id.into(),
                    PreviewPopupCallbacks {
                        close: Box::new(clone!(state => move|| {
                            state.play_course.set(None);
                        }))
                    },
                ).render(None)
            })
        })))
    }
}
