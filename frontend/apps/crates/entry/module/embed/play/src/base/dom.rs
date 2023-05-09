use crate::base::actions;
use components::module::_common::play::prelude::*;
use components::{
    backgrounds::dom::render_backgrounds_raw,
    module::_common::play::prelude::DomRenderable,
    stickers::{
        embed::dom::render_sticker_embed_raw, sprite::dom::render_sticker_sprite_raw,
        text::dom::render_sticker_text_raw,
    },
};
use dominator::{clone, html, Dom};
use shared::domain::module::body::_groups::design::{DoneAction, EmbedHost, Sticker as RawSticker};
use std::rc::Rc;

use super::state::*;

impl DomRenderable for Base {
    fn render(state: Rc<Base>) -> Dom {
        html!("empty-fragment", {
            .prop("slot", "main")
            .style("display", "contents")
            .child(
                render_backgrounds_raw(&state.backgrounds, state.theme_id, None)
            )
            .children(
                state.stickers
                    .iter()
                    .map(clone!(state => move |sticker| {
                        match sticker {
                            RawSticker::Sprite(sprite) => render_sticker_sprite_raw(sprite, None),
                            RawSticker::Text(text) => render_sticker_text_raw(text, state.theme_id, None),
                            RawSticker::Embed(embed) => {
                                let set_next_when_done = match &embed.host {
                                    EmbedHost::Youtube(youtube) if youtube.done_action == Some(DoneAction::Next) => true,
                                    _ => false,
                                };

                                let opts = actions::create_embed_sticker_options(Some(clone!(state => move || {
                                    if set_next_when_done {
                                        state.set_play_phase(ModulePlayPhase::Ending(Some(ModuleEnding::Next)));
                                    }
                                })));

                                render_sticker_embed_raw(embed, Some(opts))
                            },
                        }
                    }))
                    .collect::<Vec<Dom>>()
            )
        })
    }
}
