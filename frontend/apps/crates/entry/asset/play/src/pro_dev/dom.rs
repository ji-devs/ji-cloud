use super::actions;
use crate::pro_dev::actions::{page_forward_signal, paginate};
use components::stickers::video::ext::YoutubeUrlExt;
use dominator::{clone, html, Dom};
use futures_signals::map_ref;
use futures_signals::signal::{Mutable, Signal, SignalExt};
use serde::{Deserialize, Serialize};
use shared::domain::audio::AudioId;
use shared::domain::image::ImageId;
use shared::domain::module::body::_groups::design::VideoHost;
use shared::domain::pdf::PdfId;
use shared::domain::pro_dev::unit::{ProDevUnit, Video};
use shared::media::MediaLibrary;
use std::rc::Rc;
use utils::events;
use utils::path::{audio_lib_url, pdf_lib_url};
use web_sys::{HtmlElement, HtmlIFrameElement};

use super::state::ProDevPlayer;

// const INT_IFRAME_PADDING: usize = 30;
// const INT_INITIAL_HEIGHT: usize = 3000;

impl ProDevPlayer {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;
        actions::load_data(state.clone());

        html!("pro-dev-player", {
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
                                .text(&unit.display_name)
                            }),
                            html!("div", {
                                .text(&unit.description)
                            })
                        ])
                    })
                })
            }))
            .child_signal(state.pro_dev.signal_cloned().map(clone!(state => move |pro_dev| {
                pro_dev.map(|pro_dev| {
                    html!("div", {
                        .prop("slot", "navigation")
                        .style("display", "flex")
                        .child(html!("button", {  // Left arrow button
                            .text("<")
                            .style("order", "0")
                            .event(clone!(state => move |_: events::Click| {
                                let current_page = state.current_page.get().unwrap_or(0);
                                if current_page > 0 {
                                    state.current_page.set(Some(current_page - 1));
                                }
                                }))
                        }))
                        .child_signal(state.current_page.signal_cloned().map(clone!(
                            state, pro_dev => move |_page| {
                                Some(paginate(&state, &pro_dev))
                            }
                        )))
                        .child(html!("button", {  // Right arrow button
                            .text(">")
                            .style("order", "2")
                            .prop_signal("disable", page_forward_signal(Rc::clone(&state), &pro_dev))
                            .event(clone!(state => move |_: events::Click| {
                                    let current_page = state.current_page.get().unwrap_or(0);
                                    let num_pages = (pro_dev.pro_dev_data.units.len() + 9) / 10;
                                    if current_page < num_pages - 1 {
                                        state.current_page.set(Some(current_page + 1));
                                    }
                                }))
                            }))
                        })
                })
            })))
        })
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

    fn active_unit_signal(self: &Rc<Self>) -> impl Signal<Item = Option<ProDevUnit>> {
        map_ref! {
            let active_unit = self.active_unit.signal(),
            let pro_dev = self.pro_dev.signal_cloned() => move {
                match (active_unit, pro_dev) {
                    (Some(active_unit), Some(pro_dev)) => {
                        Some(pro_dev.pro_dev_data.units[*active_unit].clone())
                    },
                    _ => None
                }
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
                            .style("object-fit", "contain")
                            .prop("size", "full")
                            .prop("id", image.0.to_string())
                            .prop("lib", "user")
                        })
                    )
            }))
        })
    }

    pub fn render_active_link(self: &Rc<Self>, link: url::Url) -> Dom {
        // let state = self;
        html!("div", {
            .prop("slot", "player-window")
            .child(html!("iframe" => HtmlIFrameElement, {
                .style("width", "100%")
                .style("border", "none")
                .prop("src", link.to_string())
            }))
        })
    }

    pub fn render_active_pdf(self: &Rc<Self>, pdf_id: PdfId) -> Dom {
        // let state = self;
        let resp = pdf_lib_url(MediaLibrary::User, pdf_id);

        html!("div", {
            .prop("slot", "player-window")
            .child(html!("iframe" => HtmlIFrameElement, {
                .style("width", "100%")
                .style("border", "none")
                .prop("src", resp)
            }))
        })
    }

    pub fn render_active_audio(self: &Rc<Self>, audio_id: AudioId) -> Dom {
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

#[derive(Serialize, Deserialize, Debug)]
struct IframeMessageData {
    kind: String,
    height: usize,
}
