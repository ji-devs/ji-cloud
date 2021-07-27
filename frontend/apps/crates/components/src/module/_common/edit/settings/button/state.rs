use std::{fmt::Display, rc::Rc, str::FromStr};
use futures_signals::signal::{always, Mutable, Signal, SignalExt};
use std::pin::Pin;
use dominator_helpers::signals::{BoxSignalFn, box_signal_fn};

pub struct SettingsButton {
    pub(super) kind: SettingsButtonKind,
    pub(super) active_signal: BoxSignalFn<bool>,
    pub(super) value: Option<Box<dyn SettingsValueExt>>,
    pub(super) on_click: Option<Box<dyn Fn()>>
}


impl SettingsButton {
    pub fn new<S: Signal<Item = bool> + 'static>(
        kind: SettingsButtonKind, 
        active_signal: impl Fn() -> S + 'static, 
        value: Option<impl SettingsValueExt + 'static>,
        on_click: Option<impl Fn() + 'static>
    ) -> Rc<Self> {

        Rc::new(Self {
            kind,
            active_signal: box_signal_fn(active_signal),
            value: value.map(|v| Box::new(v) as _),
            on_click: on_click.map(|f| Box::new(f) as _), 
        })
    }

    //Convience helpers to avoid specifying the None casting
    pub fn new_none<S: Signal<Item = bool> + 'static>(
        kind: SettingsButtonKind, 
        active_signal: impl Fn() -> S + 'static
    ) -> Rc<Self> {
        Self::new(kind, active_signal, None::<SettingsValue<u8>>, None::<fn()>)
    }

    pub fn new_value<S: Signal<Item = bool> + 'static>(
        kind: SettingsButtonKind, 
        active_signal: impl Fn() -> S + 'static, 
        value: impl SettingsValueExt + 'static,
    ) -> Rc<Self> {
        Self::new(kind, active_signal, Some(value), None::<fn()>)
    }

    pub fn new_click<S: Signal<Item = bool> + 'static>(
        kind: SettingsButtonKind, 
        active_signal: impl Fn() -> S + 'static, 
        on_click: impl Fn() + 'static
    ) -> Rc<Self> {

        Self::new(kind, active_signal, None::<SettingsValue<u8>>, Some(on_click))
    }

    pub fn new_value_click<S: Signal<Item = bool> + 'static>(
        kind: SettingsButtonKind, 
        active_signal: impl Fn() -> S + 'static, 
        value: impl SettingsValueExt + 'static,
        on_click: impl Fn() + 'static
    ) -> Rc<Self> {
        Self::new(kind, active_signal, Some(value), Some(on_click)) 
    }
}

/** SettingsValue / SettingsValueExt allows us to ensure that
 * the pipeline is consistent for a given type like u8, u32, etc.
 */
pub trait SettingsValueExt {
    fn string_signal(&self) -> Pin<Box<dyn Signal<Item = String>>>;
    fn handle_event(&self, event_value:&str);
    fn get_select_value(&self, index: usize) -> String;
}


pub struct SettingsValue<T> {
    pub(super) curr: Mutable<T>,
    pub(super) on_change: Box<dyn Fn(T)>,
}

impl <T: Copy + Display + Eq + Default + FromStr + 'static> SettingsValue<T> {
    pub fn new(value: T, on_change: impl Fn(T) + 'static) -> Self {
        Self {
            curr: Mutable::new(value),
            on_change: Box::new(on_change)
        }
    }
}

impl <T: Copy + Display + Default + Eq + FromStr + 'static> SettingsValueExt for SettingsValue<T> {
    fn string_signal(&self) -> Pin<Box<dyn Signal<Item = String>>> {
        Box::pin(self.curr.signal().map(|value| format!("{}", value)))
    }

    fn handle_event(&self, event_value:&str) {
        let value = event_value.parse::<T>().unwrap_or_default();

        self.curr.set_neq(value);
        (self.on_change) (value);
    }

    fn get_select_value(&self, index: usize) -> String {
        format!("{}", index)
    }

}

// These must match the typescript / custom element variants
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SettingsButtonKind {
    Attempts,
    Autoplay,
    CardDouble,
    CardSingle,
    ContinueAll,
    ContinueClick,
    ContinueNextActivity,
    ContinueSome,
    Highlight,
    HighlightOff,
    Loop,
    Mute,
    NoLimit,
    NumChoices,
    NumPairs,
    Order,
    Randomize,
    Rounds,
    Score,
    ScoreOff,
    Swap,
    TimeLimit,
    TimeLimitOff,
    VideoCaptions,
}

impl SettingsButtonKind {
    pub fn as_str_id(&self) -> &'static str {
        match self {
            Self::Attempts => "attempts",
            Self::Autoplay => "autoplay",
            Self::CardDouble => "card-double",
            Self::CardSingle => "card-single",
            Self::ContinueAll => "continue-all",
            Self::ContinueClick => "continue-click",
            Self::ContinueNextActivity => "continue-next-activity",
            Self::ContinueSome => "continue-some",
            Self::Highlight => "highlight",
            Self::HighlightOff => "highlight-off",
            Self::Loop => "loop",
            Self::Mute => "mute",
            Self::NoLimit => "no-limit",
            Self::NumChoices => "n_choices",
            Self::NumPairs => "n_pairs",
            Self::Order => "order",
            Self::Randomize => "randomize",
            Self::Rounds => "rounds",
            Self::Score => "score",
            Self::ScoreOff => "score-off",
            Self::Swap => "swap",
            Self::TimeLimit => "time-limit",
            Self::TimeLimitOff => "time-limit-off",
            Self::VideoCaptions => "video-captions",
        }
    }
}
