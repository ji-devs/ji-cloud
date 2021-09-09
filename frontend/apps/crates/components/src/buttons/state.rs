use std::rc::Rc;
use utils::prelude::*;

pub struct Button {
    pub style: ButtonStyle,
    pub label: Option<String>,
    pub on_click: Option<Box<dyn Fn()>>
}

impl Button {
    pub const fn icon_str(&self) -> Option<&'static str> {
        match &self.style {
            ButtonStyle::Icon(icon) => Some(icon.as_str()),
            _ => None
        }
    }

    pub const fn element_str(&self) -> &'static str {
        match &self.style {
            ButtonStyle::Icon(_) => {
                if self.label.is_none() { "button-icon" } else { "button-icon-label" }
            },
        }
    }
}

pub enum ButtonStyle {
    Icon(ButtonStyleIcon),
}


pub enum ButtonStyleIcon {
    BlueX ,
    Audio,
    AudioStop,
    GreyKebab,
}

impl ButtonStyleIcon {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::BlueX => "circle-x-blue",
            Self::Audio => "audio",
            Self::AudioStop => "audio-stop",
            Self::GreyKebab => "circle-kebab-grey",
        }
    }
}

impl Button {
    pub fn new(style: ButtonStyle, on_click: impl Fn() + 'static) -> Rc<Self> {
        Rc::new(Self {
            style,
            label: None,
            on_click: Some(Box::new(on_click))
        })
    }
    pub fn new_label(style: ButtonStyle, label: String, on_click: impl Fn() + 'static) -> Rc<Self> {
        Rc::new(Self {
            style,
            label: Some(label),
            on_click: Some(Box::new(on_click))
        })
    }
}
