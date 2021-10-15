use components::stickers::video::dom::VideoRawRenderOptions;
use futures_signals::signal::Mutable;
use shared::domain::jig::module::body::video::{DoneAction, PlaySettings};
use std::rc::Rc;

pub fn create_video_sticker_options(
    play_settings: &PlaySettings,
    on_ended: Option<impl Fn() + 'static>,
) -> VideoRawRenderOptions {
    VideoRawRenderOptions {
        captions: Mutable::new(play_settings.captions),
        muted: Mutable::new(play_settings.muted),
        autoplay: Mutable::new(play_settings.autoplay),
        _loop: Mutable::new(matches!(play_settings.done_action, Some(DoneAction::Loop))),
        base: Default::default(),
        on_ended: on_ended.map(|f| Rc::new(f) as _),
    }
}
