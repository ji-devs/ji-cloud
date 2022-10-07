use super::state::*;
use components::{
    backgrounds::dom::render_backgrounds_raw,
    module::_common::play::prelude::DomRenderable,
    stickers::{
        dom::{render_sticker_raw, StickerRawRenderOptions},
        sprite::dom::SpriteRawRenderOptions,
        text::dom::TextRawRenderOptions,
        video::dom::VideoRawRenderOptions,
    },
};
use dominator::{apply_methods, clone, html, Dom};
use js_sys::Reflect;
use shared::domain::module::body::{
    _groups::design::Sticker as RawSticker, find_answer::QuestionField,
};
use std::rc::Rc;
use utils::unwrap::UnwrapJiExt;
use wasm_bindgen::JsValue;

use super::game::{dom::render as render_game, state::Game};

impl DomRenderable for Base {
    fn render(state: Rc<Base>) -> Dom {
        html!("empty-fragment", {
            .property("slot", "main")
            .child(render_backgrounds_raw(&state.backgrounds, state.theme_id, None))
            .child(
                // This is similar to render_stickers_raw_vec, but we need to have a reference to the text stickers so that we can update their content based on the sticker index when each question changes, if a sticker is marked as a question field.
                html!("empty-fragment", {
                    .children(
                        state.stickers
                            .iter()
                            .enumerate()
                            .map(clone!(state => move |(index, sticker)| {
                                let opts = match sticker {
                                    RawSticker::Sprite(_) => {
                                        StickerRawRenderOptions::Sprite(SpriteRawRenderOptions::default())
                                    }
                                    RawSticker::Text(_) => {
                                        let mut opts = TextRawRenderOptions::default();
                                        opts.base.set_mixin(clone!(state => move |dom| {
                                            apply_methods!(dom, {
                                                .after_inserted(clone!(state => move |elem| {
                                                    if let Some(sticker_ref) = state.sticker_refs.get(index) {
                                                        // If this is the question field sticker, then clear its content.
                                                        if let QuestionField::Text(question_index) = state.question_field {
                                                            if question_index == index {
                                                                Reflect::set(
                                                                    &elem,
                                                                    &JsValue::from_str("textValue"),
                                                                    &JsValue::from_str(" ") // This is weird. If we use "", then subsequent calls to set textValue don't work correctly.
                                                                ).unwrap_ji();
                                                            }
                                                        }

                                                        let _ = sticker_ref.set(elem);
                                                    }
                                                }))
                                            })
                                        }));
                                        StickerRawRenderOptions::Text(opts)
                                    }
                                    RawSticker::Video(_) => {
                                        StickerRawRenderOptions::Video(VideoRawRenderOptions::default())
                                    }
                                };

                                render_sticker_raw(sticker, state.theme_id, Some(opts))
                            }))
                            .collect::<Vec<Dom>>()
                    )
                })
            )
            .child(render_game(Game::new(state.clone())))
        })
    }
}
