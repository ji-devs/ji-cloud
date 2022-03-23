use dominator_helpers::signals::{box_signal_fn, BoxSignalFn};
use futures_signals::signal::Signal;
use strum_macros::Display;
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display)]
#[strum(serialize_all = "kebab-case")]
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
    #[strum(serialize = "text")]
    DualList,
    Theme,
    Tooltip,
    Video,
}
