use crate::base::state::*;
use components::stickers::{
    state::{Sticker, Stickers},
    video::state::Video,
};
use js_sys::Reflect;
use shared::domain::jig::module::body::_groups::design::VideoHost;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::HtmlElement;

impl Base {
    pub fn on_link_change(&self, host: VideoHost) {
        let stickers = self.stickers.list.lock_ref();
        let video_sticker = stickers
            .iter()
            .find(|sticker| matches!(sticker, Sticker::Video(_)));

        match video_sticker {
            None => {
                // drop stickers ref so that it can be mutated for adding the new video sticker
                drop(stickers);
                self.add_video_sticker(host);
            }
            Some(video_sticker) => {
                let video = match video_sticker {
                    Sticker::Video(video) => video,
                    _ => unreachable!("should not be possible"),
                };
                self.update_video_sticker(Rc::clone(video), host);
            }
        }
    }

    fn add_video_sticker(&self, host: VideoHost) {
        Stickers::add_video(Rc::clone(&self.stickers), host);
    }

    fn update_video_sticker(&self, sticker: Rc<Video>, host: VideoHost) {
        sticker.host.set(host);
        sticker.playing_started.set_neq(false);
        sticker.is_playing.set_neq(false);
        Stickers::call_change(&Rc::clone(&self.stickers));
    }

    pub fn delete_video(&self) {
        let mut stickers = self.stickers.list.lock_mut();
        let video_index = stickers
            .iter()
            .position(|sticker| matches!(sticker, Sticker::Video(_)));
        match video_index {
            None => log::info!("Not video to delete"),
            Some(video_index) => {
                stickers.remove(video_index);
            }
        };
    }
}

pub fn set_error(elem: &HtmlElement, error: bool) {
    let _ = Reflect::set(
        elem,
        &JsValue::from_str("error"),
        &JsValue::from_bool(error),
    );
}
