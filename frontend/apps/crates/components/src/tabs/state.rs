use dominator_helpers::signals::{box_signal_fn, BoxSignalFn};
use futures_signals::signal::{Signal, SignalExt};
use std::rc::Rc;

pub struct MenuTab {
    pub kind: MenuTabKind,
    pub sizeable: bool,
    pub active_signal: BoxSignalFn<bool>,
    pub on_click: Box<dyn Fn()>
}

impl MenuTab {
    pub fn new<A, ASig, C>(
        kind: MenuTabKind, 
        sizeable: bool,
        active_signal: A,
        on_click: C
    ) -> Rc<Self>
    where
        A: Fn() -> ASig + 'static,
        ASig: Signal<Item = bool> + 'static,
        C: Fn() + 'static
    {
        Rc::new(Self { 
            kind, 
            sizeable,
            active_signal: box_signal_fn(active_signal),
            on_click: Box::new(on_click),
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenuTabKind {
    Answer,
    Audio,
    AudioFile,
    AudioRecord,
    BackgroundColor,
    BackgroundImage,
    BackgroundImageFull,
    Color,
    Feedback,
    Image,
    Instructions,
    Overlay,
    PlaySettings,
    Question,
    Select,
    Text,
    Theme,
    Tooltip,
    AddText,
    Video,
}

impl MenuTabKind {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Answer => "answer",
            Self::Audio => "audio",
            Self::AudioFile => "audio-file",
            Self::AudioRecord => "audio-record",
            Self::BackgroundColor => "background-color",
            Self::BackgroundImage => "background-image",
            Self::BackgroundImageFull => "background-image-full",
            Self::Color => "color",
            Self::Feedback => "feedback",
            Self::Image => "image",
            Self::Instructions => "instructions",
            Self::Overlay => "overlay",
            Self::PlaySettings => "play-settings",
            Self::Question => "question",
            Self::Select => "select",
            Self::Text => "text",
            Self::Theme => "theme",
            Self::Tooltip => "tooltip",
            Self::AddText => "add-text",
            Self::Video => "video",
        }
    }
}
