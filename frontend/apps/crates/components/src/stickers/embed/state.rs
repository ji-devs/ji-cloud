use futures_signals::signal::{Mutable, Signal, SignalExt};
use shared::domain::module::body::{
    Transform,
    _groups::design::{Embed as RawEmbed, EmbedHost},
};
use std::rc::Rc;

use crate::transform::state::{TransformCallbacks, TransformState};

use super::config::{YOUTUBE_EMBED_HEIGHT, YOUTUBE_EMBED_WIDTH};

#[derive(Clone)]
pub struct Embed {
    pub host: Mutable<EmbedHost>,
    pub start_at: Mutable<Option<u32>>,
    pub end_at: Mutable<Option<u32>>,
    pub transform: Rc<TransformState>,
    pub playing_started: Mutable<bool>,
    pub is_playing: Mutable<bool>,
}

impl Embed {
    pub fn new(
        embed: &RawEmbed,
        on_transform_finished: Option<impl Fn(Transform) + 'static>,
        on_blur: Option<impl Fn() + 'static>,
    ) -> Self {
        let embed = embed.clone();
        let is_playing = Mutable::new(false);
        let playing_started = Mutable::new(false);

        let transform_callbacks =
            TransformCallbacks::new(on_transform_finished, None::<fn()>, on_blur);
        Self {
            host: Mutable::new(embed.host.clone()),
            transform: Rc::new(TransformState::new(
                embed.transform,
                Some((YOUTUBE_EMBED_WIDTH, YOUTUBE_EMBED_HEIGHT)),
                true,
                transform_callbacks,
            )),
            start_at: Mutable::new(embed.start_at),
            end_at: Mutable::new(embed.end_at),
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

    pub fn to_raw(&self) -> RawEmbed {
        RawEmbed {
            host: self.host.get_cloned(),
            transform: self.transform.get_inner_clone(),
            start_at: self.start_at.get(),
            end_at: self.end_at.get(),
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
