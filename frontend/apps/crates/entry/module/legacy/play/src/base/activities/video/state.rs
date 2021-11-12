use crate::base::state::Base;
use std::{cell::RefCell, rc::Rc};

use dominator::clone;
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt},
};
use shared::domain::jig::module::body::legacy::activity::Video as RawVideo;
use utils::{math::mat4::Matrix4, resize::resize_info_signal};
use web_sys::{HtmlElement, HtmlVideoElement};

pub struct Video {
    pub base: Rc<Base>,
    pub raw: RawVideo,
    pub start_gates: Mutable<StartGates>,
    pub video_size: Mutable<Option<(f64, f64)>>,
    pub yt_api: RefCell<Option<YoutubeApi>>,
    pub direct_api: RefCell<Option<DirectApi>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct StartGates {
    pub module: bool,
    pub video: bool,
}

impl Video {
    pub fn new(base: Rc<Base>, raw: RawVideo) -> Rc<Self> {
        let _self = Rc::new(Self {
            base,
            raw,
            video_size: Mutable::new(None),
            start_gates: Mutable::new(StartGates::default()),
            yt_api: RefCell::new(None),
            direct_api: RefCell::new(None),
        });

        _self.base.insert_start_listener(clone!(_self => move || {
            _self.clone().on_start();
        }));

        _self
    }

    pub fn has_size_signal(&self) -> impl Signal<Item = bool> {
        self.video_size.signal().map(|size| size.is_some())
    }

    pub fn first_play_signal(&self) -> impl Signal<Item = bool> {
        map_ref! {
            let start_gates = self.start_gates.signal(),
            let has_size = self.has_size_signal()
            => {
                start_gates.module && start_gates.video && *has_size
            }
        }
    }

    pub fn transform_signal(&self) -> impl Signal<Item = String> {
        let transform_matrix = match self.raw.transform_matrix {
            None => Matrix4::identity(),
            Some(m) => Matrix4::new_direct(m),
        };

        resize_info_signal().map(move |resize_info| {
            let mut m = transform_matrix.clone();
            m.denormalize(&resize_info);
            m.as_matrix_string()
        })
    }

    pub fn video_resize_signal(&self) -> impl Signal<Item = Option<(f64, f64)>> {
        map_ref! {
            let resize_info = resize_info_signal(),
            let video_size = self.video_size.signal()
            => {
                video_size.map(|video_size| resize_info.get_size_px(video_size.0, video_size.1))
            }
        }
    }

    pub fn video_offset_signal(&self) -> impl Signal<Item = Option<(f64, f64)>> {
        map_ref! {
            let resize_info = resize_info_signal(),
            let video_size = self.video_size.signal()
            => {
                video_size.map(|video_size| {
                    let (full_width, full_height) = resize_info.get_full_size_px();
                    let (width, height) = resize_info.get_size_px(video_size.0, video_size.1);

                    (
                        (full_width - width) / 2.0,
                        (full_height - height) / 2.0,
                    )
                })
            }
        }
    }
    pub fn width_signal(&self) -> impl Signal<Item = String> {
        self.video_resize_signal()
            .map(|video_size| match video_size {
                None => "0px".to_string(),
                Some((width, _)) => format!("{}px", width),
            })
    }

    pub fn height_signal(&self) -> impl Signal<Item = String> {
        self.video_resize_signal()
            .map(|video_size| match video_size {
                None => "0px".to_string(),
                Some((_, height)) => format!("{}px", height),
            })
    }

    pub fn top_signal(&self) -> impl Signal<Item = String> {
        self.video_offset_signal()
            .map(|video_offset| match video_offset {
                None => "0px".to_string(),
                Some((_, top)) => format!("{}px", top),
            })
    }
    pub fn left_signal(&self) -> impl Signal<Item = String> {
        self.video_offset_signal()
            .map(|video_offset| match video_offset {
                None => "0px".to_string(),
                Some((left, _)) => format!("{}px", left),
            })
    }

    pub fn set_yt_api(&self, elem: HtmlElement) {
        *self.yt_api.borrow_mut() = Some(YoutubeApi { elem });
    }

    pub fn set_direct_api(&self, elem: HtmlVideoElement) {
        *self.direct_api.borrow_mut() = Some(DirectApi { elem });
    }
}

pub struct YoutubeApi {
    pub elem: HtmlElement,
}

impl YoutubeApi {
    pub fn get_video_size(&self) -> (f64, f64) {
        // TODO- get from API
        let is_wide: bool = true;

        // hard-coded in TT player
        match is_wide {
            true => (527.0, 297.0),
            false => (527.0, 415.0),
        }
    }
}

pub struct DirectApi {
    pub elem: HtmlVideoElement,
}

impl DirectApi {
    pub fn get_video_size(&self) -> (f64, f64) {
        // hard-coded in TT player
        (480.0, 360.0)
    }
}
