use dominator_helpers::signals::{box_signal_fn, BoxSignalFn};
use futures_signals::signal::Signal;
use std::rc::Rc;

pub struct MenuTab {
    pub kind: MenuTabKind,
    pub sizeable: bool,
    pub enabled: bool,
    pub active_signal: BoxSignalFn<bool>,
    pub on_click: Box<dyn Fn()>,
}

impl MenuTab {
    pub fn new<A, ASig, C>(
        kind: MenuTabKind,
        sizeable: bool,
        enabled: bool,
        active_signal: A,
        on_click: C,
    ) -> Rc<Self>
    where
        A: Fn() -> ASig + 'static,
        ASig: Signal<Item = bool> + 'static,
        C: Fn() + 'static,
    {
        Rc::new(Self {
            kind,
            sizeable,
            enabled,
            active_signal: box_signal_fn(active_signal),
            on_click: Box::new(on_click),
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenuTabKind {
    Answer,
    Audio,
    BackgroundImage,
    FillColor,
    Feedback,
    Image,
    Instructions,
    Label,
    Overlay,
    PlaySettings,
    Question,
    Select,
    Text,
    Theme,
    Tooltip,
    Video,
}

impl MenuTabKind {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Answer => "answer",
            Self::Audio => "audio",
            Self::BackgroundImage => "background-image",
            Self::FillColor => "fill-color",
            Self::Feedback => "feedback",
            Self::Image => "image",
            Self::Instructions => "instructions",
            Self::Label => "label",
            Self::Overlay => "overlay",
            Self::PlaySettings => "play-settings",
            Self::Question => "question",
            Self::Select => "select",
            Self::Text => "text",
            Self::Theme => "theme",
            Self::Tooltip => "tooltip",
            Self::Video => "video",
        }
    }
}
