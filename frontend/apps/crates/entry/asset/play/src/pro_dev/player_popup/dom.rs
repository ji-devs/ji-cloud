use std::rc::Rc;

use super::PlayerPopup;
use components::{stickers::embed::types::ParseUrlExt, unit::unit_value::UnitValueView};
use dominator::{clone, html, Dom, DomBuilder};
use futures_signals::{
    map_ref,
    signal::{Signal, SignalExt},
};
use gloo::utils::{body, document};
use itertools::Itertools;
use shared::{
    domain::{
        audio::AudioId,
        image::ImageId,
        module::body::_groups::design::YoutubeEmbed,
        pdf::PdfId,
        pro_dev::{unit::{ProDevUnit, ProDevUnitValue}, ProDevResponse},
    },
    media::MediaLibrary,
};
use utils::{
    component::Component,
    events,
    path::{audio_lib_url, pdf_lib_url}, unwrap::UnwrapJiExt,
};
use web_sys::{HtmlDialogElement, HtmlElement, HtmlIFrameElement, ShadowRoot};

const UNITS_PER_PAGE: usize = 10;

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
                    .children_signal_vec(state.active_unit_signal().map(move|unit| {
                        match unit {
                            Some(unit) => vec![
                                html!("h1", {
                                    .class("name")
                                    .text(&unit.display_name)
                                }),
                                html!("p", {
                                    .class("description")
                                    .text(&unit.description)
                                })
                            ],
                            None => vec![]
                        }
                    }).to_signal_vec())
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
                .child_signal(state.player_state.pro_dev.signal_cloned().map(clone!(state => move |pro_dev| {
                    pro_dev.map(clone!(state => move |pro_dev| {
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
                            .child(state.render_unit_navigation(&pro_dev))
                            .child_signal(state.player_state.active_unit.signal_cloned().map(clone!(state, pro_dev => move |_active_unit| {
                                Some(html!("fa-button", { // forward arrow button
                                    .class("navigation-button-forward")
                                    .prop("icon", "fa-light fa-chevron-right")
                                    .prop_signal("hidden", state.navigate_forward_signal(&pro_dev))
                                    .event(clone!(state, pro_dev => move |_: events::Click| {
                                        let index = state.player_state.active_unit.get().unwrap_or(0);
                                        let current_page = state.player_state.current_page.get().unwrap_or(0);
                                        let num_pages = (pro_dev.pro_dev_data.units.len() + 9) / 10;
                                        if index < (pro_dev.pro_dev_data.units.len() - 1) {
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
                        state.player_state.active_unit.set(None);
                        state.player_state.current_page.set(None);
                    }))
                }))
            }))
        }))
    }
}

impl PlayerPopup {
    pub fn render_unit_navigation(self: &Rc<Self>, pro_dev: &Rc<ProDevResponse>) -> Dom {
        let state = self;

        let current_page = state.player_state.current_page.get().unwrap_or_default();

        let start_index = current_page * UNITS_PER_PAGE;

        let end_index = ((current_page + 1) * UNITS_PER_PAGE).min(pro_dev.pro_dev_data.units.len());

        let units_to_display = &pro_dev.pro_dev_data.units[start_index..end_index];

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

    fn active_unit_signal(self: &Rc<Self>) -> impl Signal<Item = Option<ProDevUnit>> {
        map_ref! {
            let active_unit = self.player_state.active_unit.signal(),
            let pro_dev = self.player_state.pro_dev.signal_cloned() => move {
                match (active_unit, pro_dev) {
                    (Some(active_unit), Some(pro_dev)) => {
                        Some(pro_dev.pro_dev_data.units[*active_unit].clone())
                    },
                    _ => None
                }
            }
        }
    }

    fn render_active_unit(self: &Rc<Self>, unit: ProDevUnit) -> Dom {
        html!("div", {
            .class("unit-play")
            .child(UnitValueView::new(Some(unit.value)).render())
        })
    }
}
