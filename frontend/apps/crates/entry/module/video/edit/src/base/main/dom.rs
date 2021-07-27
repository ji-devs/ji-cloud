use dominator::{Dom, clone, html};
use futures_signals::{signal::SignalExt, signal_vec::SignalVecExt};
use wasm_bindgen_futures::spawn_local;
use std::rc::Rc;
use super::state::*;
use components::{backgrounds::dom::render_backgrounds, module::_common::edit::prelude::*, stickers::{sprite::dom::render_sticker_sprite, state::Sticker, text::dom::render_sticker_text, video::dom::{VideoRenderOptions, render_sticker_video}}};

impl DomRenderable for Main {
    fn render(state: Rc<Main>) -> Dom {
        // rendering stickers manually so that video options can be passed in
        html!("empty-fragment", {
            .children_signal_vec(state.base.stickers.list
                .signal_vec_cloned()
                .enumerate()
                .map(clone!(state => move |(index, sticker)| {
                    match sticker.as_ref() {
                        Sticker::Sprite(sprite) => render_sticker_sprite(state.base.stickers.clone(), index, sprite.clone(), None),
                        Sticker::Text(text) => render_sticker_text(state.base.stickers.clone(), index, text.clone(), None),
                        Sticker::Video(video) => render_sticker_video(state.base.stickers.clone(), index, video.clone(), Some(get_video_render_opts(Rc::clone(&state)))),
                    }
                }))
            )
        })
    }
}
impl MainDomRenderable for Main {
    fn render_bg(state: Rc<Main>) -> Option<Dom> {
        Some(render_backgrounds(state.base.backgrounds.clone(), None))
    }
}

fn get_video_render_opts(state: Rc<Main>) -> VideoRenderOptions {
    VideoRenderOptions {
        captions: state.base.play_settings.captions.read_only(),
        muted: state.base.play_settings.muted.read_only(),
        done_action: state.base.play_settings.done_action.read_only(),
        ..Default::default()
    }
}
