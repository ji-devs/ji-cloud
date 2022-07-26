use super::state::*;
use components::{
    backgrounds::dom::render_backgrounds_raw,
    instructions::player::InstructionsPlayer,
    module::_common::play::prelude::{DomRenderable, ModulePlayPhase},
    stickers::{
        dom::{render_sticker_raw, StickerRawRenderOptions},
        sprite::dom::SpriteRawRenderOptions,
        text::dom::TextRawRenderOptions,
        video::dom::VideoRawRenderOptions,
    },
};
use dominator::{apply_methods, clone, html, Dom};
use futures_signals::signal::SignalExt;
use shared::domain::module::body::_groups::design::Sticker as RawSticker;
use std::rc::Rc;

use super::game::{dom::render as render_game, state::Game};

impl DomRenderable for Base {
    fn render(state: Rc<Base>) -> Dom {
        html!("empty-fragment", {
            .property("slot", "main")
            .child_signal(state.module_phase.signal_cloned().map(clone!(state => move |phase| {
                // Only play audio and update the text if we're in the playing phase.
                if let ModulePlayPhase::Playing = phase {
                    Some(InstructionsPlayer::render(state.instructions_player.clone()))
                } else {
                    None
                }
            })))
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
            .child_signal(state.instructions_finished.signal_cloned().map(clone!(state => move |finished| {
                if finished {
                    Some(render_game(Game::new(state.clone())))
                } else {
                    None
                }
            })))
        })
    }
}
