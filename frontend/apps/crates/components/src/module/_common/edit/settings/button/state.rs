use dominator_helpers::signals::{box_signal_fn, BoxSignalFn};
use futures_signals::signal::{Mutable, Signal, SignalExt};
use serde::Serialize;
use std::borrow::Cow;
use std::pin::Pin;
use std::{fmt::Display, rc::Rc, str::FromStr};

#[derive(Debug, Serialize)]
pub struct ValueLabelTemplate {
    pub prefix: Cow<'static, str>,
    pub postfix_singular: Cow<'static, str>,
    pub postfix_plural: Cow<'static, str>,
}

type TemplateTuple = (&'static str, &'static str, &'static str);

impl From<TemplateTuple> for ValueLabelTemplate {
    fn from((prefix, singular, plural): TemplateTuple) -> Self {
        Self {
            prefix: prefix.into(),
            postfix_singular: singular.into(),
            postfix_plural: plural.into(),
        }
    }
}
pub struct SettingsButton {
    pub(super) kind: SettingsButtonKind,
    pub(super) active_signal: BoxSignalFn<bool>,
    pub(super) value: Option<Box<dyn SettingsValueExt>>,
    pub(super) on_click: Option<Box<dyn Fn()>>,
    pub(super) bubble_open: Mutable<bool>,
    pub(super) tooltip: Mutable<Option<String>>,
}

pub struct SettingsButtonBuilder {
    kind: SettingsButtonKind,
    active_signal: BoxSignalFn<bool>,
    value: Option<Box<dyn SettingsValueExt>>,
    on_click: Option<Box<dyn Fn()>>,
    bubble_open: Option<Mutable<bool>>,
    tooltip: Option<Mutable<Option<String>>>,
}

impl SettingsButtonBuilder {
    /// Create a new builder with `kind` and an `active_signal` which are the minimum required
    /// fields for a [`SettingsButton`].
    pub fn new<S: Signal<Item = bool> + 'static>(
        kind: SettingsButtonKind,
        active_signal: impl Fn() -> S + 'static,
    ) -> Self {
        Self {
            kind,
            active_signal: box_signal_fn(active_signal),
            value: None,
            on_click: None,
            bubble_open: None,
            tooltip: None,
        }
    }

    /// Apply a [`SettingsValue`] to the builder
    pub fn value(mut self, value: impl SettingsValueExt + 'static) -> Self {
        self.value = Some(Box::new(value));
        self
    }

    /// Apply an on_click function to the builder
    pub fn on_click(mut self, on_click: impl Fn() + 'static) -> Self {
        self.on_click = Some(Box::new(on_click));
        self
    }

    /// Apply a boolean [`Mutable`] to the builder which will tell the [`SettingsButton`] whether
    /// the bubble is open or not. Defaults to `false`.
    pub fn bubble_open(mut self, bubble_open: Mutable<bool>) -> Self {
        self.bubble_open = Some(bubble_open);
        self
    }

    pub fn tooltip(mut self, tooltip: Mutable<Option<String>>) -> Self {
        self.tooltip = Some(tooltip);
        self
    }

    /// Build the final `SettingsButton`
    pub fn build(self) -> Rc<SettingsButton> {
        Rc::new(SettingsButton {
            kind: self.kind,
            active_signal: self.active_signal,
            value: self.value,
            on_click: self.on_click,
            bubble_open: self.bubble_open.unwrap_or_else(|| Mutable::new(false)),
            tooltip: self.tooltip.unwrap_or_else(|| Mutable::new(None)),
        })
    }
}

/** SettingsValue / SettingsValueExt allows us to ensure that
 * the pipeline is consistent for a given type like u8, u32, etc.
 */
pub trait SettingsValueExt {
    fn string_signal(&self) -> Pin<Box<dyn Signal<Item = String>>>;
    fn handle_event(&self, event_value: &str);
    fn get_select_value(&self, index: usize) -> String;
    fn get_label_template(&self) -> Option<&ValueLabelTemplate>;
    fn get_input_kind(&self) -> Option<InputKind>;
}

pub struct SettingsValue<T> {
    pub(super) curr: Mutable<T>,
    pub(super) on_change: Box<dyn Fn(T)>,
    pub(super) label_template: Option<ValueLabelTemplate>,
    pub(super) input_kind: Option<InputKind>,
}

impl<T: Copy + Display + Eq + Default + FromStr + 'static> SettingsValue<T> {
    pub fn new(value: T, on_change: impl Fn(T) + 'static) -> Self {
        Self {
            curr: Mutable::new(value),
            on_change: Box::new(on_change),
            input_kind: None,
            label_template: None,
        }
    }

    pub fn new_mutable(value: Mutable<T>, on_change: impl Fn(T) + 'static) -> Self {
        Self {
            curr: value,
            on_change: Box::new(on_change),
            input_kind: None,
            label_template: None,
        }
    }

    /// Set custom template value for the bubble label
    pub fn value_label_template(mut self, template: ValueLabelTemplate) -> Self {
        self.label_template = Some(template);
        self
    }

    /// Overwrite the default input kind for this value
    pub fn value_input_kind(mut self, input_kind: InputKind) -> Self {
        self.input_kind = Some(input_kind);
        self
    }
}

impl<T: Copy + Display + Default + Eq + FromStr + 'static> SettingsValueExt for SettingsValue<T> {
    fn string_signal(&self) -> Pin<Box<dyn Signal<Item = String>>> {
        Box::pin(self.curr.signal().map(|value| format!("{}", value)))
    }

    fn handle_event(&self, event_value: &str) {
        let value = event_value.parse::<T>().unwrap_or_default();

        self.curr.set_neq(value);
        (self.on_change)(value);
    }

    fn get_select_value(&self, index: usize) -> String {
        format!("{}", index)
    }

    fn get_label_template(&self) -> Option<&ValueLabelTemplate> {
        self.label_template.as_ref()
    }

    fn get_input_kind(&self) -> Option<InputKind> {
        self.input_kind
    }
}

// These must match the typescript / custom element variants
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SettingsButtonKind {
    Attempts,
    PlayClick,
    CardDouble,
    CardSingle,
    CardsShowAll,
    CardsShowSome,
    ContinueAll,
    ContinueClick,
    ContinueAutomatically,
    ContinueSome,
    Highlight,
    HighlightOff,
    Loop,
    Mute,
    NoLimit,
    NumChoices,
    NumPairs,
    NumPairsAlt,
    Order,
    Randomize,
    Rounds,
    Score,
    ScoreOff,
    Swap,
    TimeLimit,
    TimeLimitOff,
    VideoCaptions,
    /// Special type for overriding the label
    Custom(SettingsButtonCustomKind, &'static str),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SettingsButtonCustomKind {
    Kind(Box<SettingsButtonKind>),
    Value(&'static str),
}

impl SettingsButtonKind {
    pub fn custom_kind(kind: SettingsButtonKind, label: &'static str) -> Self {
        Self::Custom(SettingsButtonCustomKind::Kind(Box::new(kind)), label)
    }

    pub fn custom_value(value: &'static str, label: &'static str) -> Self {
        Self::Custom(SettingsButtonCustomKind::Value(value), label)
    }

    pub fn as_str_id(&self) -> &'static str {
        match self {
            Self::Attempts => "attempts",
            Self::PlayClick => "play-click",
            Self::CardDouble => "card-double",
            Self::CardSingle => "card-single",
            Self::CardsShowAll => "cards-show-all",
            Self::CardsShowSome => "cards-show-some",
            Self::ContinueAll => "continue-all",
            Self::ContinueAutomatically => "continue-automatically",
            Self::ContinueClick => "continue-click",
            Self::ContinueSome => "continue-some",
            Self::Highlight => "highlight",
            Self::HighlightOff => "highlight-off",
            Self::Loop => "loop",
            Self::Mute => "mute",
            Self::NoLimit => "no-limit",
            Self::NumChoices => "n_choices",
            Self::NumPairs => "n_pairs",
            Self::NumPairsAlt => "n_pairs-alt",
            Self::Order => "order",
            Self::Randomize => "randomize",
            Self::Rounds => "rounds",
            Self::Score => "score",
            Self::ScoreOff => "score-off",
            Self::Swap => "swap",
            Self::TimeLimit => "time-limit",
            Self::TimeLimitOff => "time-limit-off",
            Self::VideoCaptions => "video-captions",
            Self::Custom(id, _) => match id {
                SettingsButtonCustomKind::Kind(kind) => kind.as_str_id(),
                SettingsButtonCustomKind::Value(value) => *value,
            },
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum InputKind {
    Field,
    Select(usize),
}

pub fn get_input_kind(kind: &SettingsButtonKind) -> Option<InputKind> {
    // If the kind is a CustomKind, then we need to check whether the variant is Kind first. If it is
    // return that so we can determine whether or not show an input field on it.
    let kind = match kind {
        SettingsButtonKind::Custom(SettingsButtonCustomKind::Kind(kind), _) => &*kind,
        _ => kind,
    };

    match kind {
        SettingsButtonKind::Attempts => Some(InputKind::Select(6)),
        SettingsButtonKind::NumChoices => Some(InputKind::Select(6)),
        SettingsButtonKind::NumPairs => Some(InputKind::Field),
        SettingsButtonKind::NumPairsAlt => Some(InputKind::Select(6)),
        SettingsButtonKind::TimeLimit => Some(InputKind::Field),
        SettingsButtonKind::ContinueSome => Some(InputKind::Field),
        SettingsButtonKind::Rounds => Some(InputKind::Field),
        SettingsButtonKind::CardsShowSome => Some(InputKind::Field),
        _ => None,
    }
}
