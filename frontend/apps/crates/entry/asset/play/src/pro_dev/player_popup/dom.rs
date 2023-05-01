use std::rc::Rc;

use super::PlayerPopup;
use components::stickers::video::ext::YoutubeUrlExt;
use dominator::{clone, html, Dom, DomBuilder};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt},
};
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
            .after_inserted(|dialog: HtmlDialogElement| {
                let _ = dialog.show_modal();
            })
            .prop("slot", "dialog")
            .child(html!("pro-dev-player", {
                .child_signal(state.active_unit_signal().map(clone!(state => move|unit| {
                    unit.map(|unit| {
                        state.render_active_unit(unit)
                    })
                })))
                .child_signal(state.active_unit_signal().map(move|unit| {
                    unit.map(|unit| {
                        html!("div", {
                            .prop("slot", "title")
                            .children(&mut [
                                html!("div", {
                                    .style("text-align", "center")
                                    .text(&unit.display_name)
                                }),
                                html!("div", {
                                    .style("text-align", "center")
                                    .text(&unit.description)
                                })
                            ])
                        })
                    })
                }))
                .child_signal(state.player_state.pro_dev.signal_cloned().map(clone!(state => move |pro_dev| {
                    pro_dev.map(clone!(state => move |pro_dev| {
                        html!("div", {
                            .prop("slot", "navigation")
                            .style("display", "flex")
                            .child_signal(state.player_state.active_unit.signal_cloned().map(clone!(state=> move |_active_unit| {
                                Some(
                                    html!("button", {  // back arrow button
                                        .text("<")
                                        .style("order", "0")
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
                            .child_signal(state.player_state.current_page.signal_cloned().map(clone!(
                                state, pro_dev => move |_page| {
                                    Some(state.paginate(&pro_dev))
                                }
                            )))
                            .child_signal(state.player_state.active_unit.signal_cloned().map(clone!(state, pro_dev => move |_active_unit| {
                                Some(
                                    html!("button", {  // forward arrow button
                                        .text(">")
                                        .style("order", "2")
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
                    .prop("slot", "close")
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
    pub fn paginate(self: &Rc<Self>, pro_dev: &Rc<ProDevResponse>) -> Dom {
        let state = self;

        let units_per_page = 10;

        let current_page = state.player_state.current_page.get().unwrap_or(0);

        let start_index = current_page * units_per_page;

        let end_index = ((current_page + 1) * units_per_page).min(pro_dev.pro_dev_data.units.len());

        let units_to_display = &pro_dev.pro_dev_data.units[start_index..end_index];

        // TODO: actions.rs should not render html
        // Create buttons for each unit on the current page
        let unit_buttons =
            units_to_display
                .iter()
                .enumerate()
                .map(clone!(state => move |(index, _unit)| {
                    html!("button", {
                        .text(&((current_page * units_per_page) + index + 1).to_string())
                        .event(clone!(state, index => move |_: events::Click| {
                            state.player_state.active_unit.set(Some(current_page * units_per_page + index));
                        }))
                    })
                }));

        html!("div", {
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
        let played_units = self.player_state.played_units.get_cloned();
        if !played_units.contains(&index) {
            self.player_state.played_units.lock_mut().insert(index);
        }
    }

    fn render_active_unit(self: &Rc<Self>, unit: ProDevUnit) -> Dom {
        match unit.value {
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
        }
    }

    fn render_active_video(self: &Rc<Self>, video: Video) -> Dom {
        let mut_host = Mutable::new(video.host.clone());

        html!("div", {
            .prop("slot", "player-window")
            .child_signal(mut_host.signal_cloned().map(move|host| {
                match host {
                    VideoHost::Youtube(youtube_video) => Some(
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
                            .style("display", "block")
                            .style("width", "100%")
                            .style("height", "100%")
                        })
                    ),
                }
            }))
        })
    }

    fn render_active_image(self: &Rc<Self>, image: ImageId) -> Dom {
        let mut_image = Mutable::new(image.clone());

        html!("div", {
            .prop("slot", "player-window")
            .child_signal(mut_image.signal_cloned().map(move|image| {
                    Some(
                        html!("img-ji", {
                            // would like to get rid if the styles here
                            .style("position", "relative")
                            .style("object-fit", "contain")
                            .prop("size", "full")
                            .prop("id", image.0.to_string())
                            .prop("lib", "user")
                        })
                    )
            }))
        })
    }

    fn render_active_link(self: &Rc<Self>, link: url::Url) -> Dom {
        // let state = self;
        html!("div", {
            .prop("slot", "player-window")
            .child(html!("iframe" => HtmlIFrameElement, {
                .style("width", "100%")
                .style("height", "100%")
                .style("border", "none")
                .prop("src", link.to_string())
            }))
        })
    }

    fn render_active_pdf(self: &Rc<Self>, pdf_id: PdfId) -> Dom {
        // let state = self;
        let resp = pdf_lib_url(MediaLibrary::User, pdf_id);

        html!("div", {
            .prop("slot", "player-window")
            .child(html!("iframe" => HtmlIFrameElement, {
                .style("width", "100%")
                .style("height", "100%")
                .style("position", "relative")
                .prop("src", resp)
            }))
        })
    }

    fn render_active_audio(self: &Rc<Self>, audio_id: AudioId) -> Dom {
        // let state = self;
        let resp = audio_lib_url(MediaLibrary::User, audio_id);

        html!("div", {
            .prop("slot", "player-window")
            .child(html!("audio", {
                .style("width", "100%")
                .style("border", "none")
                .prop("src", resp)
                .prop("controls", true)
            }))
        })
    }
}
