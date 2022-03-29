use futures_signals::signal::{Mutable, Signal, SignalExt};
use shared::domain::jig::module::body::{
    Transform,
    _groups::design::{Video as RawVideo, VideoHost},
};
use std::rc::Rc;

use crate::transform::state::{TransformCallbacks, TransformState};

use super::config::{YOUTUBE_VIDEO_HEIGHT, YOUTUBE_VIDEO_WIDTH};

#[derive(Clone)]
pub struct Video {
    pub host: Mutable<VideoHost>,
    pub transform: Rc<TransformState>,
    pub playing_started: Mutable<bool>,
    pub is_playing: Mutable<bool>,
}

impl Video {
    pub fn new(
        video: &RawVideo,
        on_transform_finished: Option<impl Fn(Transform) + 'static>,
        on_blur: Option<impl Fn() + 'static>,
    ) -> Self {
        let video = video.clone();
        let is_playing = Mutable::new(false);
        let playing_started = Mutable::new(false);

        let transform_callbacks =
            TransformCallbacks::new(on_transform_finished, None::<fn()>, on_blur);
        Self {
            host: Mutable::new(video.host.clone()),
            transform: Rc::new(TransformState::new(
                video.transform,
                Some((YOUTUBE_VIDEO_WIDTH, YOUTUBE_VIDEO_HEIGHT)),
                true,
                transform_callbacks,
            )),
            playing_started,
            is_playing,
        }
    }

    pub fn width_signal(&self) -> impl Signal<Item = String> {
        width_signal(self.transform.size.signal_cloned())
    }
    pub fn height_signal(&self) -> impl Signal<Item = String> {
        height_signal(self.transform.size.signal_cloned())
    }

    pub fn to_raw(&self) -> RawVideo {
        RawVideo {
            host: self.host.get_cloned(),
            transform: self.transform.get_inner_clone(),
        }
    }
}

pub fn width_signal(size: impl Signal<Item = Option<(f64, f64)>>) -> impl Signal<Item = String> {
    size.map(|size| match size {
        None => "0".to_string(),
        Some(size) => format!("{}rem", size.0),
    })
}

pub fn height_signal(size: impl Signal<Item = Option<(f64, f64)>>) -> impl Signal<Item = String> {
    size.map(|size| match size {
        None => "0".to_string(),
        Some(size) => format!("{}rem", size.1),
    })
}
