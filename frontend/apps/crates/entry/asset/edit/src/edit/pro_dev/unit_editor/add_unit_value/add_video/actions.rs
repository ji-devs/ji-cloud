use std::rc::Rc;

use js_sys::Reflect;
use shared::{
    api::endpoints,
    domain::{
        audio::{user::UserAudioCreatePath, AudioId},
        module::body::_groups::design::VideoHost,
        pro_dev::unit::{ProDevUnitValue, Video},
    },
};
use utils::{prelude::ApiEndpointExt, unwrap::UnwrapJiExt};
use wasm_bindgen::JsValue;
use web_sys::{Blob, File, HtmlElement};

use super::state::AddVideo;

impl AddVideo {
    pub fn save(self: &Rc<Self>, host: VideoHost) {
        let state = Rc::clone(self);

        self.add_unit_value_state.loader.load(async move {
            let video = Video {
                host,
                start_at: None,
                end_at: None,
            };

            state
                .add_unit_value_state
                .unit_editor_state
                .value
                .set(Some(ProDevUnitValue::Video(video)));
        })
    }

    // pub fn on_link_change(&self, host: VideoHost) {
    //     let video = self.get_video_sticker();

    //     match video {
    //         None => {
    //             self.add_video_sticker(host);
    //         }
    //         Some(video) => {
    //             self.update_video_sticker(video, host);
    //         }
    //     }
    // }

    // #[must_use]
    // pub fn get_video_sticker(&self) -> Option<Rc<Video>> {
    //     let stickers = self.stickers.list.lock_ref();

    //     let video = stickers
    //         .iter()
    //         .find(|sticker| matches!(sticker, Sticker::Video(_)))
    //         .map(|sticker| match sticker {
    //             Sticker::Video(video) => video,
    //             _ => unreachable!("should not be possible"),
    //         });

    //     let video = video.map(|video| Rc::clone(&video));

    //     video
    // }

    // pub fn delete_video(&self) {
    //     let mut video = self.video;

    //     match video {
    //         None => log::info!("Cannot delete video"),
    //         Some(video) => {
    //             self.video.set(None);
    //         }
    //     };
    // }
}

pub fn set_error(elem: &HtmlElement, error: bool) {
    let _ = Reflect::set(
        elem,
        &JsValue::from_str("error"),
        &JsValue::from_bool(error),
    );
}
