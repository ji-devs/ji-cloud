use std::rc::Rc;

use components::overlay::handle::OverlayHandle;
use dominator::{class, clone, html, pseudo, with_node, Dom};
use futures_signals::{map_ref, signal::SignalExt};
use shared::{domain::circle::Circle, media::MediaLibrary};
use utils::{
    events,
    routes::{CommunityCirclesRoute, CommunityRoute, Route},
};
use web_sys::HtmlInputElement;

use crate::state::CIRCLE_LIST_GRID_COLUMNS;

use super::{create_circle::CreateCircle, CirclesList};

impl CirclesList {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;
        state.load_circles();

        html!("community-list", {
            .property("header", "Join a circle")
            .child(html!("button-rect", {
                .property("slot", "create-button")
                .property("color", "blue")
                .text("Start a new circle")
                .event(clone!(state => move |_: events::Click| {
                    state.create_popup_open.set(true);
                }))
            }))
            .child(html!("community-list-circle-header", {
                .class(&*CIRCLE_LIST_GRID_COLUMNS)
                .property("slot", "sort-header")
            }))
            .child(html!("community-pagination", {
                .property("slot", "sort-header")
                .property_signal("total", state.total_pages.signal())
                .children(&mut [
                    html!("fa-button", {
                        .property("slot", "back")
                        .property("icon", "fa-solid fa-angle-left")
                        .property_signal("disabled", state.active_page.signal().map(|active_page| {
                            active_page <= 1
                        }))
                        .event(clone!(state => move |_: events::Click| {
                            let active_page = state.active_page.get();
                            if active_page > 1 {
                                state.active_page.set(active_page - 1);
                                state.load_circles();
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
                            .property("slot", "active-page")
                            .property("type", "number")
                            .property("min", 1)
                            .property_signal("max", state.total_pages.signal())
                            .property_signal("value", state.active_page.signal().map(|active_page| {
                                active_page.to_string()
                            }))
                            .event(clone!(state, elem => move |_: events::Input| {
                                let value = elem.value();
                                if let Ok(num) = value.parse::<u32>() {
                                    if num <= state.total_pages.get() {
                                        state.active_page.set(num);
                                        state.load_circles();
                                    }
                                };
                            }))
                        })
                    }),
                    html!("fa-button", {
                        .property("slot", "forward")
                        .property("icon", "fa-solid fa-angle-right")
                        .property_signal("disabled", map_ref! {
                            let active_page = state.active_page.signal(),
                            let total_pages = state.total_pages.signal() => {
                                active_page >= total_pages
                            }
                        })
                        .event(clone!(state => move |_: events::Click| {
                            state.active_page.replace_with(|active_page| {
                                *active_page + 1
                            });
                            state.load_circles();
                        }))
                    }),
                ])
            }))
            .children_signal_vec(state.circles.signal_ref(clone!(state => move|circles| {
                match circles {
                    None => {
                        vec![html!("progress", {
                            .property("slot", "items")
                        })]
                    },
                    Some(circles) => {
                        circles.iter().map(|circle| {
                            state.render_circle(circle)
                        }).collect()
                    },
                }
            })).to_signal_vec())
            .child_signal(state.create_popup_open.signal().map(clone!(state => move |open| {
                match open {
                    false => None,
                    true => {
                        Some(html!("empty-fragment", {
                            .apply(OverlayHandle::lifecycle(clone!(state => move || {
                                CreateCircle::new(Rc::clone(&state)).render()
                            })))
                        }))
                    },
                }
            })))
        })
    }

    fn render_circle(self: &Rc<Self>, circle: &Circle) -> Dom {
        html!("community-list-circle", {
            .class(&*CIRCLE_LIST_GRID_COLUMNS)
            .property("slot", "items")
            .property("name", &circle.display_name)
            .property("memberCount", circle.member_count)
            .property("description", &circle.description)
            .apply(move |dom| dominator::on_click_go_to_url!(dom, {
                Route::Community(CommunityRoute::Circles(CommunityCirclesRoute::Circle(circle.id))).to_string()
            }))
            .child(html!("img-ji", {
                .property("slot", "img")
                .property("lib", MediaLibrary::User.to_str())
                .property("id", &circle.image.0.to_string())
            }))
            .child(html!("community-list-circle-status", {
                .property("slot", "status")
                .property("status", "")
            }))
        })
    }
}
