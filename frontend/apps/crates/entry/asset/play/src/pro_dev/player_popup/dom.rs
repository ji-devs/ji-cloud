use std::rc::Rc;

use super::PlayerPopup;
use components::stickers::video::ext::YoutubeUrlExt;
use dominator::{clone, html, Dom, DomBuilder};
use futures_signals::{
    map_ref,
    signal::{Signal, SignalExt},
};
use gloo::utils::{document, body};
use itertools::Itertools;
use shared::{
    domain::{
        audio::AudioId,
        image::ImageId,
        module::body::_groups::design::VideoHost,
        pdf::PdfId,
        pro_dev::{
            unit::{ProDevUnit, Video},
            ProDevResponse,
        },
    },
    media::MediaLibrary,
};
use utils::{
    component::Component,
    events,
    path::{audio_lib_url, pdf_lib_url},
};
use web_sys::{HtmlDialogElement, HtmlElement, HtmlIFrameElement, ShadowRoot};

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
                                false => "fa-regular fa-arrows-maximize",
                                true => "fa-regular fa-arrows-minimize",
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
                                            state.played_unit(index);
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
                                            state.played_unit(index);
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
                    }))
                }))
            }))
        }))
    }
}

impl PlayerPopup {
    pub fn render_unit_navigation(self: &Rc<Self>, pro_dev: &Rc<ProDevResponse>) -> Dom {
        let state = self;

        let units_per_page = 10;

        let current_page = state.player_state.current_page.get().unwrap_or(0);

        let start_index = current_page * units_per_page;

        let end_index = ((current_page + 1) * units_per_page).min(pro_dev.pro_dev_data.units.len());

        let units_to_display = &pro_dev.pro_dev_data.units[start_index..end_index];

        // Create buttons for each unit on the current page
        let unit_buttons = units_to_display
            .iter()
            .enumerate()
            .map(clone!(state => move |(index, _unit)| {
                html!("button", {
                    .class("unit-navigation-button")
                    .class_signal("active", state.player_state.active_unit.signal().map(move |active_unit| {
                        active_unit.map(|active_unit| active_unit == index).unwrap_or_default()
                    }))
                    .class_signal("done", state.player_state.played_units.signal_ref(move |played_units| {
                        played_units.contains(&index)
                    }))
                    .text(&((current_page * units_per_page) + index + 1).to_string())
                    .event(clone!(state => move |_: events::Click| {
                        state.player_state.active_unit.set(Some(current_page * units_per_page + index));
                        state.played_unit(index);
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

    fn played_unit(self: &Rc<Self>, index: usize) {
        self.player_state.played_units.lock_mut().insert(index);
    }

    fn render_active_unit(self: &Rc<Self>, unit: ProDevUnit) -> Dom {
        html!("div", {
            .class("player-window")
            .child(match unit.value {
                shared::domain::pro_dev::unit::ProDevUnitValue::ImageId(image_id) => {
                    self.render_active_image(image_id)
                }
                shared::domain::pro_dev::unit::ProDevUnitValue::AudioId(audio_id) => {
                    self.render_active_audio(audio_id)
                }
                shared::domain::pro_dev::unit::ProDevUnitValue::Link(url) => {
                    self.render_active_link(url)
                }
                shared::domain::pro_dev::unit::ProDevUnitValue::PdfId(pdf_id) => {
                    self.render_active_pdf(pdf_id)
                }
                shared::domain::pro_dev::unit::ProDevUnitValue::Video(video) => {
                    self.render_active_video(video)
                }
            })
        })
    }

    fn render_active_video(self: &Rc<Self>, video: Video) -> Dom {
        match video.host {
            VideoHost::Youtube(youtube_video) => {
                html!("video-youtube-player" => HtmlElement, {
                    .prop("videoId", youtube_video.url.get_id())
                    .apply(|mut dom| {
                        if let Some(start_at) = video.start_at {
                            dom = dom.prop("start", start_at);
                        }
                        if let Some(end_at) = video.end_at {
                            dom = dom.prop("end", end_at);
                        }
                        dom
                    })
                })
            }
        }
    }

    fn render_active_image(self: &Rc<Self>, image: ImageId) -> Dom {
        html!("img-ji", {
            // would like to get rid if the styles here
            .prop("size", "full")
            .prop("id", image.0.to_string())
            .prop("lib", "user")
        })
    }

    fn render_active_link(self: &Rc<Self>, link: url::Url) -> Dom {
        html!("iframe" => HtmlIFrameElement, {
            .prop("src", link.to_string())
        })
    }

    fn render_active_pdf(self: &Rc<Self>, pdf_id: PdfId) -> Dom {
        let resp = pdf_lib_url(MediaLibrary::User, pdf_id);

        html!("iframe" => HtmlIFrameElement, {
            .prop("src", resp)
        })
    }

    fn render_active_audio(self: &Rc<Self>, audio_id: AudioId) -> Dom {
        let resp = audio_lib_url(MediaLibrary::User, audio_id);

        html!("audio", {
            .prop("src", resp)
            .prop("controls", true)
        })
    }
}
