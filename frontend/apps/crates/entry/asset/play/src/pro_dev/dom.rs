use super::actions;
use components::stickers::video::ext::YoutubeUrlExt;
use dominator::{clone, html, Dom};
use futures_signals::map_ref;
use futures_signals::signal::{Mutable, Signal, SignalExt};
use serde::{Serialize, Deserialize};
use shared::domain::image::ImageId;
use shared::domain::module::body::_groups::design::VideoHost;
use shared::domain::pdf::PdfId;
use shared::domain::pro_dev::unit::{ProDevUnit, Video};
use shared::media::MediaLibrary;
use utils::path::pdf_lib_url;
use std::rc::Rc;
use utils::events;
use web_sys::{HtmlElement, HtmlIFrameElement};

use super::state::ProDevPlayer;

// const INT_IFRAME_PADDING: usize = 30;
// const INT_INITIAL_HEIGHT: usize = 3000;

impl ProDevPlayer {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;
        actions::load_data(state.clone());

        log::info!("After load");

        html!("div", {
            // start stupid
            .text_signal(state.active_unit.signal().map(|active_unit| {
                format!("Active unit: {active_unit:?}")
            }))
            .child(html!("br"))
            // end stupid
            .child_signal(state.active_unit_signal().map(clone!(state => move|unit| {
                unit.map(|unit| {
                    state.render_active_unit(unit)
                })
            })))
            .child_signal(state.pro_dev.signal_cloned().map(clone!(state => move |pro_dev| {
                pro_dev.map(|pro_dev| {
                    log::info!("Inside html");

                    html!("div", {
                        .text(format!("display name: {}", pro_dev.pro_dev_data.display_name).as_str())
                        .children(pro_dev.pro_dev_data.units.iter().enumerate().map(clone!(state => move |(index, unit)| {
                            html!("button", {
                                .text(&unit.display_name)
                                .event(clone!(state, index => move |_: events::Click| {
                                    state.active_unit.set(Some(index))
                                }))
                            })
                        })))
                    })               
                })
            })))
        })
    }

    fn render_active_unit(self: &Rc<Self>, unit: ProDevUnit) -> Dom {
        match unit.value {
            shared::domain::pro_dev::unit::ProDevUnitValue::ImageId(image_id) => self.render_active_image(image_id),
            shared::domain::pro_dev::unit::ProDevUnitValue::AudioId(_) => todo!("<audio> tag"),
            shared::domain::pro_dev::unit::ProDevUnitValue::Link(url) => self.render_active_link(url),
            shared::domain::pro_dev::unit::ProDevUnitValue::PdfId(pdf_id) => self.render_active_pdf(pdf_id),
            shared::domain::pro_dev::unit::ProDevUnitValue::Video(video) => self.render_active_video(video),
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
                            // .event(clone!(on_ended => move |_: events::YoutubeEnded| {
                            //     if let Some(on_ended) = on_ended.as_ref() {
                            //         (on_ended) ();
                            //     }
                            // }))
                        })
                    ),
                }
            }))
        })
    }

    fn render_active_image(self: &Rc<Self>, image: ImageId) -> Dom {
        let mut_image = Mutable::new(image.clone());

        html!("div", {
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
        html!("iframe" => HtmlIFrameElement, {
            .style("width", "100%")
            .style("border", "none")
            // .global_event(clone!(state => move |event: Message| {
            //     if let Ok(data) = event.try_serde_data::<IframeMessageData>() {
            //         if data.kind == "scrollHeight" {
            //             state.height.set(data.height + INT_IFRAME_PADDING);
            //         }
            //     }
            // }))
            .prop("src", link.to_string())
        })
    }
    
    pub fn render_active_pdf(self: &Rc<Self>, pdf_id: PdfId) -> Dom {
        // let state = self;
        let resp = pdf_lib_url(MediaLibrary::User, pdf_id);
        html!("iframe" => HtmlIFrameElement, {
            .style("width", "100%")
            .style("border", "none")
            .prop("src", resp)
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct IframeMessageData {
    kind: String,
    height: usize,
}
