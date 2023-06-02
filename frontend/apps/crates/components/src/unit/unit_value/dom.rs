use std::rc::Rc;

use crate::stickers::embed::types::ParseUrlExt;
use dominator::{clone, html, Dom, DomBuilder};
use futures_signals::signal::SignalExt;
use shared::{
    domain::{
        audio::AudioId, course::unit::CourseUnitValue, image::ImageId,
        module::body::_groups::design::YoutubeEmbed, pdf::PdfId,
    },
    media::MediaLibrary,
};
use utils::{
    component::Component,
    path::{audio_lib_url, pdf_lib_url},
    unwrap::UnwrapJiExt,
};
use web_sys::{File, HtmlElement, HtmlIFrameElement, ShadowRoot, Url};

use super::UnitValueView;

impl Component<UnitValueView> for Rc<UnitValueView> {
    fn styles() -> &'static str {
        include_str!("./styles.css")
    }

    fn apply_on_host(&self, dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
        dom.class("unit-play").prop("slot", "unit-play")
    }

    fn dom(&self, dom: DomBuilder<ShadowRoot>) -> DomBuilder<ShadowRoot> {
        let state = self;

        dom.child(html!("div", {
            .child_signal(state.unit_value.signal_cloned().map(clone!(state => move |unit_value| {
                Some(match unit_value {
                    Some(value) => state.render(value),
                    None => html!("div", {}),
                })
            })))
        }))
    }
}

impl UnitValueView {
    fn render(self: &Rc<Self>, unit: CourseUnitValue) -> Dom {
        html!("div", {
            .class("player-window")
            .child(match unit {
                shared::domain::course::unit::CourseUnitValue::ImageId(image_id) => {
                    self.render_active_image(image_id)
                }
                shared::domain::course::unit::CourseUnitValue::AudioId(audio_id) => {
                    self.render_active_audio(audio_id)
                }
                shared::domain::course::unit::CourseUnitValue::Link(url) => {
                    self.render_active_link(url)
                }
                shared::domain::course::unit::CourseUnitValue::PdfId(pdf_id) => {
                    self.render_active_pdf(pdf_id)
                }
                shared::domain::course::unit::CourseUnitValue::Video(video) => {
                    self.render_active_video(video)
                }
            })
        })
    }

    fn render_active_video(self: &Rc<Self>, video: YoutubeEmbed) -> Dom {
        html!("video-youtube-player" => HtmlElement, {
            .prop("videoId", video.url.get_id())
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

    fn render_active_image(self: &Rc<Self>, image: ImageId) -> Dom {
        html!("img-ji", {
            // would like to get rid if the styles here
            // .prop("size", "full")
            .prop("id", image.0.to_string())
            .prop("lib", "user")
            .prop("borderRadius", "16px")
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

pub fn file_to_object_url(file: &File) -> String {
    Url::create_object_url_with_blob(file).unwrap_ji()
}
