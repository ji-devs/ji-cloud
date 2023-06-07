use std::rc::Rc;

use super::PlayerPopup;
use components::unit::unit_value::UnitValueView;
use dominator::{clone, html, Dom, DomBuilder};
use futures_signals::{
    map_ref,
    signal::{Signal, SignalExt},
};
use gloo::utils::{body, document};
use itertools::Itertools;
use shared::domain::course::{unit::CourseUnit, CourseResponse};
use utils::{
    component::Component,
    events,
    js_wrappers::is_iframe,
    prelude::{AssetPlayerToPlayerPopup, IframeAction, IframeMessageExt},
    unwrap::UnwrapJiExt,
};
use web_sys::{HtmlDialogElement, ShadowRoot};

const UNITS_PER_PAGE: usize = 10;
const STR_READ_MORE: &str = "Read more";
const MAX_CHAR_LEN: usize = 100;

impl Component<PlayerPopup> for Rc<PlayerPopup> {
    fn styles() -> &'static str {
        include_str!("./styles.css")
    }

    fn dom(&self, dom: DomBuilder<ShadowRoot>) -> DomBuilder<ShadowRoot> {
        let state = self;
        dom.child(html!("dialog" => HtmlDialogElement, {
            .prop("open", true)
            .prop("slot", "dialog")
            .child(html!("main", {
                .child_signal(state.active_unit_signal().map(clone!(state => move|unit| {
                    unit.map(|unit| {
                        state.render_active_unit(unit)
                    })
                })))
                .child(html!("div", {
                    .class("middle-section")
                    .children_signal_vec(state.active_unit_signal().map(clone!(state => move|unit| {
                        match unit {
                            Some(unit) => {
                                let description= if &unit.description.len() >= &MAX_CHAR_LEN {
                                    state.read_more.set(true);
                                    state.name.set(Some(unit.display_name.clone()));
                                    state.description.set(Some(unit.description.clone()));
                                    &unit.description[..MAX_CHAR_LEN]
                                }else {
                                    state.read_more.set(false);
                                    unit.description.as_str()
                                };

                                vec![
                                html!("h1", {
                                    .class("name")
                                    .text(&unit.display_name)
                                }),
                                html!("p", {
                                    .children(&mut [
                                        html!("div", {
                                            .class("description")
                                            .text(description)
                                        }),
                                        html!("button-empty", {
                                            .class("read-more")
                                            .text_signal(state.read_more_signal().map(move |read_more| {
                                                if let Some(read_more) = read_more {
                                                    format!("  {}", read_more)
                                                } else {
                                                    format!(" ")
                                                }
                                            }))
                                            .event(clone!(state => move|_: events::Click| {
                                                state.render_popup.set(true)
                                            }))
                                        }),
                                    ])
                                }),
                            ]},
                            None => vec![]
                        }
                    })).to_signal_vec())
                    .child_signal(state.render_popup.signal_cloned().map(clone!(state => move |popup| {
                        match popup {
                            true => Some(state.render_info_popup()),
                            false => None,
                        }
                    })))
                    .child(html!("fa-button", {
                        .class("fullscreen-button")
                        .prop_signal("icon", state.is_full_screen.signal().map(|is_full_screen| {
                            match is_full_screen {
                                false => "fa-light fa-arrows-maximize",
                                true => "fa-light fa-arrows-minimize",
                            }
                        }))
                        .event(clone!(state => move|_: events::Click| {
                            match state.is_full_screen.get() {
                                true => {
                                    let _ = document().exit_fullscreen();
                                },
                                false => {
                                    let _ = body().request_fullscreen();
                                },
                            };
                        }))
                        .global_event(clone!(state => move|_: events::FullScreenChange| {
                            let is_full_screen = document().fullscreen_element().is_some();
                            state.is_full_screen.set(is_full_screen);
                        }))

                    }))
                }))
                .child_signal(state.player_state.course.signal_cloned().map(clone!(state => move |course| {
                    course.map(clone!(state => move |course| {
                        html!("div", {
                            .class("bottom-bar")
                            .child_signal(state.player_state.active_unit.signal_cloned().map(clone!(state=> move |_active_unit| {
                                Some(html!("fa-button", { // back arrow button
                                    .class("navigation-button-back")
                                    .prop("icon", "fa-light fa-chevron-left")
                                    .prop_signal("hidden", state.navigate_previous_signal())
                                    .event(clone!(state => move |_: events::Click| {
                                        let index = state.player_state.active_unit.get().unwrap_or(0);
                                        let current_page = state.player_state.current_page.get().unwrap_or(0);
                                        if index > 0 {
                                            state.player_state.played_units.lock_mut().insert(index - 1);
                                            state.player_state.active_unit.set(Some(index - 1));
                                            if (index + 1) % 10 == 1 && current_page > 0 {
                                                state.player_state.current_page.set(Some(current_page - 1));
                                            }
                                        }
                                    }))
                                }))
                            })))
                            .child(state.render_unit_navigation(&course))
                            .child_signal(state.player_state.active_unit.signal_cloned().map(clone!(state, course => move |_active_unit| {
                                Some(html!("fa-button", { // forward arrow button
                                    .class("navigation-button-forward")
                                    .prop("icon", "fa-light fa-chevron-right")
                                    .prop_signal("hidden", state.navigate_forward_signal(&course))
                                    .event(clone!(state, course => move |_: events::Click| {
                                        let index = state.player_state.active_unit.get().unwrap_or(0);
                                        let current_page = state.player_state.current_page.get().unwrap_or(0);
                                        let num_pages = (course.course_data.units.len() + 9) / 10;
                                        if index < (course.course_data.units.len() - 1) {
                                            state.player_state.played_units.lock_mut().insert(index + 1);
                                            state.player_state.active_unit.set(Some(index + 1));
                                            if (index + 1) % 10 == 0  && (current_page < (num_pages - 1))  {
                                                state.player_state.current_page.set(Some(current_page + 1));
                                            }
                                        }
                                    }))
                                }))
                            })))
                        })
                    }))
                })))
                .child(html!("fa-button", {
                    .class("close")
                    .prop("icon", "fa-light fa-xmark")
                    .event(clone!(state => move |_: events::Click| {
                        if is_iframe() {
                            let _ = IframeAction::new(AssetPlayerToPlayerPopup::CloseButtonShown(true))
                                .try_post_message_to_parent();
                        }
                        state.player_state.active_unit.set(None);
                        state.player_state.current_page.set(None);
                    }))
                }))

            }))
        }))
    }
}

impl PlayerPopup {
    pub fn render_unit_navigation(self: &Rc<Self>, course: &Rc<CourseResponse>) -> Dom {
        let state = self;

        let current_page = state.player_state.current_page.get().unwrap_or_default();

        let start_index = current_page * UNITS_PER_PAGE;

        let end_index = ((current_page + 1) * UNITS_PER_PAGE).min(course.course_data.units.len());

        let units_to_display = &course.course_data.units[start_index..end_index];

        // Create buttons for each unit on the current page
        let unit_buttons = units_to_display
            .iter()
            .enumerate()
            .map(clone!(state => move |(index, _unit)| {
                let global_index = current_page * UNITS_PER_PAGE + index;

                html!("button", {
                    .class("unit-navigation-button")
                    .class_signal("active", state.player_state.active_unit.signal().map(move |active_unit| {
                        active_unit.map(|active_unit|
                        active_unit == global_index
                        ).unwrap_or_default()
                    }))
                    .class_signal("done", state.player_state.played_units.signal_ref(move |played_units| {
                        played_units.contains(&global_index)
                    }))
                    .text(&((current_page * UNITS_PER_PAGE) + index + 1).to_string())
                    .event(clone!(state => move |_: events::Click| {
                        state.player_state.played_units.lock_mut().insert(global_index);
                        state.player_state.active_unit.set(Some(global_index));
                    }))
                })
            }))
            .collect_vec();

        html!("div", {
            .class("unit-navigation-container")
            .children(
                unit_buttons
            )
        })
    }

    pub fn render_info_popup(self: &Rc<Self>) -> Dom {
        let state = self;
        html!("popup-info", {
            .children(&mut [
                html!("fa-button", {
                    .prop("icon", "fa-light fa-xmark")
                    .event(clone!(state => move |_: events::Click| {
                        state.render_popup.set(false)
                    }))
                }),
                html!("div", {
                    .class("popup-name")
                    .text_signal(state.name_signal().map(move |name| name.unwrap_ji()))
                }),
                html!("div", {
                    .class("popup-description")
                    .text_signal(state.description_signal().map(move |description| description.unwrap_ji()))
                }),
                html!("button-empty", {
                    .class("popup-close")
                    .text("Close")
                    .event(clone!(state => move |_: events::Click| {
                        state.render_popup.set(false)
                    }))
                })
            ])
        })
    }

    fn active_unit_signal(self: &Rc<Self>) -> impl Signal<Item = Option<CourseUnit>> {
        map_ref! {
            let active_unit = self.player_state.active_unit.signal(),
            let course = self.player_state.course.signal_cloned() => move {
                match (active_unit, course) {
                    (Some(active_unit), Some(course)) => {
                        Some(course.course_data.units[*active_unit].clone())
                    },
                    _ => None
                }
            }
        }
    }

    fn read_more_signal(self: &Rc<Self>) -> impl Signal<Item = Option<String>> {
        map_ref! {
            let read_more = self.read_more.signal_cloned() => move {
                match read_more {
                    true => Some(STR_READ_MORE.to_string()),
                    false => None,
                }
            }
        }
    }

    fn name_signal(self: &Rc<Self>) -> impl Signal<Item = Option<String>> {
        self.name.signal_cloned().map(move |name| match name {
            Some(name) => Some(name),
            None => None,
        })
    }

    fn description_signal(self: &Rc<Self>) -> impl Signal<Item = Option<String>> {
        self.description
            .signal_cloned()
            .map(move |description| match description {
                Some(description) => Some(description),
                None => None,
            })
    }

    fn render_active_unit(self: &Rc<Self>, unit: CourseUnit) -> Dom {
        html!("div", {
            .class("unit-play")
            .child(UnitValueView::new(Some(unit.value)).render())
        })
    }
}
