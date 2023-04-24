use components::stickers::embed::dom::EmbedRawRenderOptions;
use std::rc::Rc;

pub fn create_embed_sticker_options(
    on_ended: Option<impl Fn() + 'static>,
) -> EmbedRawRenderOptions {
    EmbedRawRenderOptions {
        base: Default::default(),
        on_ended: on_ended.map(|f| Rc::new(f) as _),
    }
}
