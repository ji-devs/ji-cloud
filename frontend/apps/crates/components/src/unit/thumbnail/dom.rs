use crate::stickers::embed::types::ParseUrlExt;

use super::state::*;
use dominator::{html, Dom};
use shared::domain::image::ImageId;
use shared::domain::{module::body::_groups::design::YoutubeEmbed, pro_dev::unit::ProDevUnitValue};
use std::rc::Rc;
use utils::prelude::*;

impl UnitThumbnail {
    pub fn render_live(self: Rc<Self>, slot: Option<&str>) -> Dom {
        let state = self;

        html!("div", {
            .apply_if(slot.is_some(), |dom| {
                dom.prop("slot", slot.unwrap_ji())
            })
            .child_signal(state.unit_value.signal_ref(|unit_value| {
                let thumbnail = if let Some(unit) = unit_value {
                    match unit {
                        ProDevUnitValue::ImageId(image) => render_image_thumbnail(image),
                        ProDevUnitValue::AudioId(_) => render_audio_thumbnail(),
                        ProDevUnitValue::Link(_) => render_link_thumbnail(),
                        ProDevUnitValue::PdfId(_) => render_pdf_thumbnail(),
                        ProDevUnitValue::Video(youtube) => render_youtube_thumbnail(youtube),
                    }
                } else {
                    None
                };

                thumbnail
            }))
        })
    }
}

fn render_youtube_thumbnail(youtube: &YoutubeEmbed) -> Option<Dom> {
    Some(html!("video-youtube-thumbnail", {
        .prop("videoId", youtube.url.get_id().to_owned())
        .style("width", "100%")
        .style("height", "100%")
        .style("position", "relative")
        .style("border-radius", "10px")
    }))
}

fn render_pdf_thumbnail() -> Option<Dom> {
    Some(html!("img-ui", {
        .prop("path", "entry/pro-dev/thumbnail/thumbnail-pdf.svg")
        .style("position", "absolute")
    }))
}

fn render_audio_thumbnail() -> Option<Dom> {
    Some(html!("img-ui", {
        .prop("path", "entry/pro-dev/thumbnail/thumbnail-audio.svg")
        .style("position", "absolute")
    }))
}

fn render_link_thumbnail() -> Option<Dom> {
    Some(html!("img-ui", {
        .prop("path", "entry/pro-dev/thumbnail/thumbnail-link.svg")
        .style("position", "absolute")
    }))
}

fn render_image_thumbnail(image: &ImageId) -> Option<Dom> {
    Some(html!("img-ji", {
        .style("display", "flex")
        .style("justify-content", "center")
        .style("width", "100%")
        .style("height", "100%")
        .style("object-fit", "cover")
        .style("align-items", "center")
        .prop("borderRadius", "10px")
        .prop("id", image.0.to_string())
        .prop("lib", "user")
    }))
}
